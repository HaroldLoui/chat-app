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
};

export const API_CONFIG_APIS = {
  LIST: "plugin:chat-openai|list_api_config",
  ADD: "plugin:chat-openai|insert_api_config",
  SET_DEFAULT: "plugin:chat-openai|set_default_config",
  DEL: "plugin:chat-openai|delete_api_config",
};

export const GLOBAL_CONFIG_APIS = {
  QUERY_STREAM: "plugin:chat-openai|query_enable_stream",
  UPDATE_STREAM: "plugin:chat-openai|update_enable_stream",
  QUERY_CONTEXT: "plugin:chat-openai|query_associated_context",
  UPDATE_CONTEXT: "plugin:chat-openai|update_associated_context",
};

export const PROXY_APIS = {
  QUERY_PROXY: "plugin:chat-request|query_proxy",
  UPDATE_PROXY: "plugin:chat-request|update_proxy",
  ENABLE_PROXY: "plugin:chat-request|enable_proxy",
  CHECK_PROXY: "plugin:chat-request|check_proxy",
};

export const EVENT_NAME = {
  MESSAGE: "chat:message://received",
  STREAM_MESSAGE: "chat:message://stream_received",
};
