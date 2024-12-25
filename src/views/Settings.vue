<template>
  <div class="container">
    <div class="back-btn">
      <vs-button color="primary" type="flat" @click="goBack">
        <i class="bx bx-arrow-back"></i>
      </vs-button>
    </div>
    <div class="main">
      <div class="form-box">
        <div class="form-title">代理设置</div>
        <div class="form-item">
          <div class="form-item-label">Host</div>
          <div class="form-item-input">
            <vs-input v-model="proxyForm.host" style="width: 100%" placeholder="http://127.0.0.1" />
          </div>
        </div>
        <div class="form-item">
          <div class="form-item-label">Port</div>
          <div class="form-item-input">
            <vs-input v-model="proxyForm.port" type="number" placeholder="6789" />
          </div>
        </div>
        <div class="form-item">
          <vs-checkbox v-model="proxyForm.authentication"> Proxy authentication </vs-checkbox>
        </div>
        <div class="form-item">
          <div class="form-item-label" :class="{ diabled: !proxyForm.authentication }">Username</div>
          <div class="form-item-input">
            <vs-input v-model="proxyForm.username" :disabled="!proxyForm.authentication" style="width: 100%" />
          </div>
        </div>
        <div class="form-item">
          <div class="form-item-label" :class="{ diabled: !proxyForm.authentication }">Password</div>
          <div class="form-item-input">
            <vs-input
              v-model="proxyForm.password"
              :disabled="!proxyForm.authentication"
              type="password"
              style="width: 100%"
            />
          </div>
        </div>
        <div class="form-item">
          <vs-button color="primary" type="flat" @click="onSaveProxy(true)"> 保存并启用 </vs-button>
          <vs-button color="primary" type="flat" @click="onSaveProxy(false)"> 保存 </vs-button>
          <vs-button v-if="proxyForm.isEnable" color="danger" type="flat" @click="onEnableProxy(false)">
            禁用代理
          </vs-button>
          <vs-button v-else color="success" type="flat" @click="onEnableProxy(true)"> 启用代理 </vs-button>
        </div>
      </div>
      <div class="form-box">
        <div class="form-title">接口管理</div>
        <div class="form-item">
          <vs-button color="primary" type="flat" @click="onInsertConfig"> 新增 </vs-button>
        </div>
        <div class="form-item" v-for="config in apiConfigs" :key="config.id" style="gap: 10px">
          <div style="width: 40%">
            <vs-input style="width: 100%" v-model="config.url" readonly label="接口地址" />
          </div>
          <div style="width: 40%">
            <vs-input style="width: 100%" v-model="config.key" readonly label="密钥" type="password" />
          </div>
          <div class="btns">
            <text-button :disabled="config.isDefault" @click="handleConfig(config.id, true)">设为默认</text-button>
            <text-button type="danger" @click="handleConfig(config.id, false)">删除</text-button>
          </div>
        </div>
        <div v-if="showConfigForm" class="form-item" style="gap: 10px">
          <div style="width: 40%">
            <vs-input style="width: 100%" v-model="configForm.url" label="接口地址" />
          </div>
          <div style="width: 40%">
            <vs-input style="width: 100%" v-model="configForm.key" label="密钥" type="password" />
          </div>
          <div class="btns">
            <text-button @click="handleSaveConfig(true)">保存并默认</text-button>
            <text-button @click="handleSaveConfig(false)">仅保存</text-button>
          </div>
        </div>
      </div>
    </div>
  </div>
  <my-confirm ref="myConfirm" @confirm="invokeConfig"></my-confirm>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { MESSAGE_APIS } from "../constants";

import TextButton from "../components/TextButton.vue";
import MyConfirm from "../components/Confirm.vue";

const router = useRouter();
const goBack = () => {
  router.back();
};

onMounted(() => {
  queryProxy();
});

const proxyForm = ref<Proxy>({
  id: 0,
  host: "http://127.0.0.1",
  port: 10809,
  username: "",
  password: "",
  isEnable: false,
  authentication: false,
});
const queryProxy = async () => {
  proxyForm.value = await invoke(MESSAGE_APIS.QUERY_PROXY);
};
const onSaveProxy = async (enable: boolean) => {
  const form: Proxy = { ...proxyForm.value };
  if (enable) {
    form.isEnable = enable;
  }
  await invoke(MESSAGE_APIS.UPDATE_PROXY, { entity: form });
  queryProxy();
};
const onEnableProxy = async (enable: boolean) => {
  await invoke(MESSAGE_APIS.ENABLE_PROXY, { enable });
  queryProxy();
};

const apiConfigs = reactive<Array<ApiConfig>>([
  {
    id: 1,
    url: "https://asdsadsadadsaopenai.com",
    key: "123",
    isDefault: true,
  },
  {
    id: 2,
    url: "https://openai.com",
    key: "123",
    isDefault: false,
  },
]);
const configForm = reactive<ApiConfig>({
  id: 0,
  url: null,
  key: null,
  isDefault: false,
});
const showConfigForm = ref<boolean>(false);
const onInsertConfig = () => {
  configForm.id = 0;
  configForm.url = null;
  configForm.key = null;
  configForm.isDefault = false;
  showConfigForm.value = true;
};
const handleSaveConfig = (setDefault: boolean) => {
  configForm.isDefault = setDefault;
  console.log("保存成功");
  showConfigForm.value = false;
};

const myConfirm = ref();
const chooseConfig = ref<number | null>();
const handleType = ref<boolean>(true);
const handleConfig = (id: number, type: boolean) => {
  chooseConfig.value = id;
  handleType.value = type;
  const msg = type ? "确认设置该配置为默认配置？" : "删除后不可恢复，确定删除当前接口配置？";
  myConfirm.value.show(msg);
};
const invokeConfig = () => {
  if (chooseConfig.value !== null) {
    if (handleType.value) {
      console.log("调用设置默认接口", chooseConfig.value);
    } else {
      console.log("调用删除接口", chooseConfig.value);
    }
    chooseConfig.value = null;
  }
};
</script>

<style lang="scss" scoped>
.container {
  width: 100%;
  position: relative;

  .back-btn {
    position: absolute;
    top: 10px;
    left: 10px;
  }

  .main {
    width: 80%;
    margin: 0 auto;
    height: 100vh;
    padding: 50px;
    background-color: #fff;
    box-shadow:
      -5px 0 10px #ccc,
      5px 0 10px #ccc;
    display: flex;
    flex-direction: column;
    gap: 10px;
    align-items: center;

    .form-box {
      width: 90%;
      min-height: 80px;
      border: 1px solid #ccc;
      border-radius: 10px;
      position: relative;
      padding: 15px;
      margin-bottom: 20px;

      &:last-child {
        margin-bottom: 0;
      }

      .form-title {
        position: absolute;
        top: -13px;
        left: 10px;
        background-color: #fff;
        color: #686868;
        text-align: center;
        padding: 0 10px;
        cursor: default;
      }

      .form-item {
        width: 100%;
        min-height: 40px;
        margin-bottom: 10px;
        display: flex;
        align-items: center;
        justify-content: flex-start;

        &:last-child {
          margin-bottom: 0;
        }

        .form-item-label {
          width: 80px;
          color: #333;
          font-size: 16px;
        }

        .diabled {
          color: #ccc;
        }

        .form-item-input {
          margin-left: 10px;
          width: calc(100% - 70px);
        }

        .btns {
          width: 20%;
          display: flex;
          height: 100%;
          align-items: center;
          justify-content: space-around;
          padding-top: 26px;
        }
      }
    }
  }
}
</style>
