<template>
  <div class="siderbar-header">
    <div class="title">Chat App</div>
    <div class="search">
      <vs-input
        style="width: 100%"
        state="primary"
        v-model="searchValue"
        icon-after
        placeholder="搜索聊天..."
        @input="handleSearch"
      >
        <template #icon>
          <i class="bx bx-search"></i>
        </template>
      </vs-input>
    </div>
  </div>
  <div class="siderbar-main">
    <div class="chat-list">
      <ChatBox
        v-for="(chat, i) in chatList"
        :key="i"
        :value="chat"
        :active="activeIndex === i"
        @choose="handleChooseChat(i, chat)"
        @delete="handleDeleteChat"
      ></ChatBox>
    </div>
  </div>
  <div class="siderbar-footer">
    <div class="btns">
      <vs-button type="flat" @click="onSetting">
        <i class="bx bx-cog"></i>
        <template #animate> 设置 </template>
      </vs-button>
      <vs-button type="flat">
        <i class="bx bxl-github"></i>
        <template #animate> 源码 </template>
      </vs-button>
      <vs-button type="flat" color="success" animation-type="scale" style="width: 100px" @click="handleAddChat">
        <i class="bx bx-plus"></i>
        <template #animate> 添加聊天 </template>
      </vs-button>
    </div>
  </div>

  <my-confirm ref="myConfirm" @confirm="invokeDeleteChat"></my-confirm>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { CHAT_BOX_APIS } from "../constants";

import ChatBox from "../components/ChatBox.vue";
import MyConfirm from "../components/Confirm.vue";

const props = defineProps({
  value: {
    type: Number,
    required: true,
  },
});
watch(
  () => props.value,
  () => {
    handleSearch();
  }
);

onMounted(async () => {
  await handleSearch();
  if (chatList.value.length > 0) {
    emits("changeChat", chatList.value[0]);
  } else {
    emits("changeChat", null);
  }
});

const searchValue = ref<string>("");
const handleSearch = async () => {
  await getChatList();
};

const chatList = ref<ChatBox[]>([]);
const activeIndex = ref<number>(0);
const getChatList = async () => {
  chatList.value = await invoke(CHAT_BOX_APIS.LIST, { title: searchValue.value });
};

const handleAddChat = async () => {
  await invoke(CHAT_BOX_APIS.ADD);
  getChatList();
};

const emits = defineEmits(["changeChat"]);
const handleChooseChat = (i: number, chat: ChatBox) => {
  activeIndex.value = i;
  emits("changeChat", chat);
};

const myConfirm = ref();
const deleteChatId = ref<string>("");
const handleDeleteChat = (id: string | number) => {
  deleteChatId.value = id.toString();
  myConfirm.value.show("删除后不可恢复！确认删除该对话？");
};
const invokeDeleteChat = async () => {
  await invoke(CHAT_BOX_APIS.DEL, { id: deleteChatId.value });
  getChatList();
  myConfirm.value.close();
};

const router = useRouter();
const onSetting = () => {
  router.push("/setting");
};
</script>

<style lang="scss" scoped>
.siderbar-header {
  height: 100px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-around;

  .title {
    height: 50px;
    line-height: 50px;
    text-align: center;
    font-size: 28px;
    font-weight: bolder;
  }

  .search {
    width: 90%;
  }
}

.siderbar-main {
  flex: 1;
  overflow-x: hidden;
  overflow-y: auto;

  .chat-list {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
  }

  &::-webkit-scrollbar {
    width: 0;
  }
}

.siderbar-footer {
  height: 60px;

  .btns {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: space-around;
  }
}

.not-margin {
  margin: 0px;
  font-weight: normal;
  padding: 10px;
}

.con-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  width: calc(100%);
  .new {
    margin: 0px;
    margin-top: 20px;
    padding: 0px;
    font-size: 0.7rem;
    a {
      color: rgba(var(--vs-primary)) !important;
      margin-left: 6px;
      &:hover {
        text-decoration: underline;
      }
    }
  }
  .vs-button {
    margin: 0px;
  }
}
</style>
