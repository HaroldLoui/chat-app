<template>
  <div class="app-container">
    <div class="app-siderbar">
      <Siderbar :value="num" @change-chat="handleChangeChat"></Siderbar>
    </div>
    <div class="app-main">
      <Main v-if="activeChat" :value="activeChat" @change="onMessageChange"></Main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { open } from "@tauri-apps/plugin-shell";
import Siderbar from "../layout/Siderbar.vue";
import Main from "../layout/Main.vue";

onMounted(() => {
  openUrlAction();
});

const openUrlAction = () => {
  document.addEventListener("DOMContentLoaded", () => {
    document.querySelectorAll("a").forEach((link) => {
      link.addEventListener("click", (e) => {
        e.preventDefault();
        const url = link.getAttribute("href");
        if (isUrl(url)) {
          open(url!);
        }
      });
    });
  });
};

const isUrl = (url: string | null) => {
  if (!url) {
    return false;
  }
  return url.startsWith("https://") || url.startsWith("http://");
};

const activeChat = ref<ChatBox | null>(null);
const handleChangeChat = (value: ChatBox) => {
  if (value) {
    activeChat.value = value;
  }
};
const num = ref<number>(0);
const onMessageChange = () => {
  num.value = (num.value + 1) % 10;
};
</script>

<style scoped lang="scss">
.app-container {
  width: 100vw;
  height: 100vh;
  display: flex;

  .app-siderbar {
    width: 35%;
    max-width: 300px;
    height: 100%;
    border-right: 1px solid #ccc;
    display: flex;
    flex-direction: column;
    justify-content: space-around;
  }

  .app-main {
    width: 100%;
    height: 100%;
  }
}
</style>
