export const CHAT_BOX_APIS = {
  LIST: "list_chat_box",
  ADD: "insert_chat_box",
  EDIT_TITLE: "update_chat_box_title",
  DEL: "delete_chat_box",
};

export const MESSAGE_APIS = {
  LIST_MESSAGE: "list_message",
  ADD_MESSAGE: "insert_message",
  SEND_MESSAGE: "plugin:chat-openai|send_message",
  QUERY_PROXY: "plugin:chat-request|query_proxy",
  UPDATE_PROXY: "plugin:chat-request|update_proxy",
  ENABLE_PROXY: "plugin:chat-request|enable_proxy",
};
