<template>
  <div class="app-container">
    <div class="app-siderbar">
      <Siderbar @change-chat="handleChangeChat"></Siderbar>
    </div>
    <div class="app-main">
      <Main :value="activeChat"></Main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
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

const getNow = () => {
  const date = new Date();
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  const hours = String(date.getHours()).padStart(2, "0");
  const minutes = String(date.getMinutes()).padStart(2, "0");
  const seconds = String(date.getSeconds()).padStart(2, "0");

  return `${year}/${month}/${day} ${hours}:${minutes}:${seconds}`;
};

const blankChat = reactive<ChatBox>({
  id: "",
  title: "默认对话",
  count: 0,
  createTime: getNow(),
});
const activeChat = ref<ChatBox>(blankChat);
const handleChangeChat = (value: ChatBox) => {
  if (value) {
    activeChat.value = value;
  }
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
