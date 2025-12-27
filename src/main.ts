import { createApp } from "vue";

import App from "./App.vue";
import router from "./router";

import "./rpc-bridge";

createApp(App).use(router).mount("#app");
