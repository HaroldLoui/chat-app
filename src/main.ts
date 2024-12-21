import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

import "./style.css";
// import "./assets/boxicons.min.css";

import Vuesax from "vuesax-alpha";
import "vuesax-alpha/theme-chalk/index.css";
// dark mode
// import "vuesax-alpha/theme-chalk/dark/css-vars.css";
import * as VuesaxAlphaIconsVue from "@vuesax-alpha/icons-vue";

const app = createApp(App);
for (const [key, component] of Object.entries(VuesaxAlphaIconsVue)) {
  app.component(`VsIcon${key}`, component as any);
}

app.use(Vuesax);
app.use(router);

app.mount("#app");
