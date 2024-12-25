declare type ButtonType = "primary" | "success" | "warning" | "danger" | "info";

declare interface ChatBox {
  id: string;
  title: string;
  count: number;
  createTime: string;
}

declare interface Message {
  id: string;
  chatId: string;
  sender: "AI" | "ME";
  content: string;
  createTime: string;
}

declare interface Proxy {
  id: number;
  host: string | null;
  port: number | null;
  username: string | null;
  password: string | null;
  isEnable: boolean;
  authentication: boolean;
}

declare interface ApiConfig {
  id: number;
  url: string | null;
  key: string | null;
  isDefault: boolean;
}
