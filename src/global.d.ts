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
  proxyAddress?: string;
  host: string;
  port: number;
  username: string;
  password: string;
  isEnable: boolean;
}