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
      <!-- <div v-if="hasNextPage" class="load-more" @click="onLoadMore">加载更多...</div> -->
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
        <textarea
          v-model="senderInfo"
          class="content-input"
          :placeholder="sendPlaceholder"
          v-on:keydown="onCtrlEnterDown"
        ></textarea>
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
import { nextTick, onMounted, onUnmounted, reactive, ref, watch } from "vue";
import MessageBox from "../components/MessageBox.vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { Store } from "@tauri-apps/plugin-store";
import { CHAT_BOX_APIS, MESSAGE_APIS, EVENT_NAME } from "../constants";

const canSetScrollTop = ref<boolean>(true);
const props = defineProps<{
  value: ChatBox;
}>();
watch(
  () => props.value.id,
  () => {
    messageList.value = [];
    getMessageList();
    canSetScrollTop.value = false;
    setTimeout(() => {
      canSetScrollTop.value = true;
    }, 1000);
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
onMounted(() => {
  const content = document.getElementById("scrollableContent")!;
  contentDom.value = content;
  listenReceived();
  listenScroll(content);
  messageList.value = [];
  getMessageList();
});
const unlisten = ref<Promise<UnlistenFn>>();
const unlistenStream = ref<Promise<UnlistenFn>>();
onUnmounted(async () => {
  if (unlisten.value) {
    const f = unlisten.value;
    (await f)();
  }
  if (unlistenStream.value) {
    const f = unlistenStream.value;
    (await f)();
  }
});
const streamContent = ref<string>("");
const listenReceived = () => {
  unlisten.value = listen<string>(EVENT_NAME.MESSAGE, async (event) => {
    await insertMessage("AI", event.payload, props.value.id);
  });
  unlistenStream.value = listen<string>(EVENT_NAME.STREAM_MESSAGE, async (event) => {
    if (event.payload === "DONE") {
      await insertMessage("AI", streamContent.value, props.value.id);
      streamContent.value = "";
      aiMessage.content = "思考中...";
    } else {
      streamContent.value += event.payload;
      aiMessage.content = streamContent.value;
      nextTick(() => {
        if (contentDom.value) {
          scrollToBottom(contentDom.value);
        }
      });
    }
  });
};

const scrollToBottom = (content: HTMLElement) => {
  setTimeout(() => {
    content.scrollTop = content.scrollHeight;
  }, 200);
};
const listenScroll = async (content: HTMLElement) => {
  const store = await Store.load("store.json", {
    autoSave: true,
  });
  content.addEventListener("scroll", () => {
    if (canSetScrollTop.value) {
      store.set(props.value.id.toString(), content.scrollTop);
    }
  });
};

const messageList = ref<Message[]>([]);
const getMessageList = async () => {
  messageList.value = await invoke(MESSAGE_APIS.LIST_MESSAGE, { chat_id: props.value.id });
  const store = await Store.get("store.json");
  if (store) {
    const top = await store.get<number>(props.value.id.toString());
    if (contentDom.value && top !== undefined) {
      contentDom.value.style.scrollBehavior = "auto";
      contentDom.value.scrollTop = top;
      nextTick(() => {
        contentDom.value!.style.scrollBehavior = "smooth";
      });
      return;
    }
  }
  nextTick(() => {
    if (contentDom.value) {
      scrollToBottom(contentDom.value);
    }
  });
};

const sendPlaceholder = ref<string>("按“Ctrl + Enter”进行发送，“Enter”换行");
const onCtrlEnterDown = (event: KeyboardEvent) => {
  if (event.ctrlKey && event.key === "Enter") {
    onSendMessage();
  }
};
const senderInfo = ref<string>("");
const loading = ref<boolean>(false);
const aiMessage = reactive<Message>({
  id: "",
  chatId: props.value.id,
  sender: "AI",
  content: "思考中...",
  createTime: "",
});
const onSendMessage = async () => {
  const content = senderInfo.value;
  if (!content) {
    return;
  }
  const chatId = props.value.id;
  const sender = "ME";
  await insertMessage(sender, content, chatId);
  senderInfo.value = "";
  loading.value = true;
  messageList.value.push(aiMessage);
  await invoke(MESSAGE_APIS.SEND_MESSAGE, { content, chat_id: chatId });
  loading.value = false;
};

const insertMessage = async (sender: "AI" | "ME", content: string, chatId: string) => {
  console.log("insert: ", sender, content);
  const message = await invoke<Message>(MESSAGE_APIS.ADD_MESSAGE, { chat_id: chatId, content, sender });
  if (sender === "ME") {
    messageList.value.push(message);
  } else {
    messageList.value[messageList.value.length - 1] = message;
  }
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
