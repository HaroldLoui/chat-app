<template>
  <div class="chat-room">
    <div class="chat-title-box">
      <div class="title">
        <vs-input
          v-if="editMode"
          ref="updateTitleInput"
          v-model="value.title"
          input-style="border"
          style="height: 100%; margin-top: 10px"
          @keyup.enter.native="handleEditTitle"
        />
        <span v-else>{{ value.title }}</span>
      </div>
      <div class="buttons">
        <vs-button icon color="success" type="border" @click="enterEditMode">
          <i class="bx bx-edit"></i>
        </vs-button>
      </div>
    </div>
    <div class="chat-messages" id="scrollableContent">
      <div v-if="hasNextPage" class="load-more" @click="onLoadMore">加载更多...</div>
      <MessageBox v-for="message in messageList" :key="message.id" :value="message"></MessageBox>
    </div>
    <div class="chat-sender">
      <div class="sender-toolbar">
        <vs-button icon color="#909399" type="border">
          <i class="bx bx-smile"></i>
        </vs-button>
        <vs-button icon color="#909399" type="border">
          <i class="bx bxs-image"></i>
        </vs-button>
        <vs-button icon color="#909399" type="border">
          <i class="bx bxs-invader"></i>
        </vs-button>
      </div>
      <div class="sender-box">
        <textarea v-model="senderInfo" class="content-input"></textarea>
        <div class="sender-btn">
          <vs-button :loading="loading" type="relief" @click="onSendMessage">
            <i class="bx bxs-paper-plane"></i>
            发送
          </vs-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import MessageBox from "../components/MessageBox.vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { CHAT_BOX_APIS, MESSAGE_APIS } from "../constants";

const props = defineProps<{
  value: ChatBox;
}>();
watch(
  () => props.value.id,
  () => {
    console.log("213123");
    pageNum.value = 1;
    messageList.value = [];
    getMessageList();
  }
);

const editMode = ref<boolean>(false);
const updateTitleInput = ref();
const enterEditMode = () => {
  editMode.value = true;
  nextTick(() => {
    updateTitleInput.value.focus();
  });
};
const handleEditTitle = async () => {
  const id = props.value.id;
  const title = props.value.title;
  await invoke(CHAT_BOX_APIS.EDIT_TITLE, { id: id.toString(), title });
  editMode.value = false;
};

const contentDom = ref<HTMLElement>();
const hasNextPage = ref<boolean>(false);
onMounted(() => {
  const content = document.getElementById("scrollableContent")!;
  contentDom.value = content;
  nextTick(() => {
    scrollToBottom(content);
    hasNextPage.value = true;
  });
  listenReceived();
  pageNum.value = 1;
  messageList.value = [];
  getMessageList();
});
const unlisten = ref<Promise<UnlistenFn>>();
onUnmounted(async () => {
  if (unlisten.value) {
    const f = unlisten.value;
    (await f)();
  }
});
const listenReceived = () => {
  unlisten.value = listen<string>("chat:message://received", async (event) => {
    await insertMessage("AI", event.payload, props.value.id);
  });
};

const scrollToBottom = (content: HTMLElement) => {
  setTimeout(() => {
    content.scrollTop = content.scrollHeight;
  }, 200);
};

const messageList = ref<Message[]>([]);
const pageNum = ref<number>(1);
const onLoadMore = () => {
  pageNum.value += 1;
  getMessageList();
};
const getMessageList = async () => {
  const list: Message[] = await invoke(MESSAGE_APIS.LIST_MESSAGE, { chat_id: props.value.id, page_num: pageNum.value });
  hasNextPage.value = list.length >= 10;
  messageList.value = list.concat(messageList.value);
};

const senderInfo = ref<string>("");
const loading = ref<boolean>(false);
const onSendMessage = async () => {
  const chatId = props.value.id;
  const content = senderInfo.value;
  const sender = "ME";
  await insertMessage(sender, content, chatId);
  senderInfo.value = "";
  loading.value = true;
  await invoke(MESSAGE_APIS.SEND_MESSAGE, { content, chat_id: chatId });
  loading.value = false;
};

const insertMessage = async (sender: "AI" | "ME", content: string, chatId: string) => {
  const message = await invoke<Message>(MESSAGE_APIS.ADD_MESSAGE, { chat_id: chatId, content, sender });
  messageList.value.push(message);
  if (contentDom.value) {
    scrollToBottom(contentDom.value);
  }
};
</script>

<style lang="scss" scoped>
.chat-room {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-around;

  .chat-title-box {
    display: flex;
    height: 60px;
    background-color: #fff;
    border-bottom: 1px solid #ccc;

    .title {
      width: 70%;
      height: 60px;
      line-height: 60px;
      font-size: 20px;
      font-weight: bolder;
      padding-left: 20px;
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
    }

    .buttons {
      width: 30%;
      height: 60px;
      display: flex;
      justify-content: flex-end;
      align-items: center;
      padding-right: 10px;
    }
  }

  .chat-messages {
    flex: 1;
    padding-left: 10px;
    padding-right: 10px;
    overflow-y: auto;
    scroll-behavior: smooth;

    &::-webkit-scrollbar {
      width: 0;
    }

    .load-more {
      text-align: center;
      font-size: 12px;
      color: rgb(var(--vs-success-light-3));
      cursor: pointer;

      &:hover {
        color: rgb(var(--vs-success-light-7));
      }
    }
  }

  .chat-sender {
    height: 180px;
    border-top: 1px solid #ccc;
    background-color: #fff;

    .sender-toolbar {
      height: 50px;
      display: flex;
      align-items: center;
      justify-content: flex-start;
    }

    .sender-box {
      position: relative;
      padding: 10px;
      padding-top: 0;

      .content-input {
        height: 110px;
        display: inline-block;
        padding: 10px 90px 10px 10px;
        width: 100%;
        border-color: #ccc;
        border-radius: 5px;
        resize: none;
        &::-webkit-scrollbar {
          width: 0;
        }
        &:focus {
          border-color: rgba(var(--vs-primary-light-5));
        }
      }

      .sender-btn {
        position: absolute;
        bottom: 25px;
        right: 10px;
        width: 90px;
        height: 40px;
      }
    }
  }
}
</style>
