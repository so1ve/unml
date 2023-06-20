import { createApp } from "vue";

import "uno.css";
import "@unocss/reset/tailwind.css";
import "./styles/main.scss";

import App from "./App.vue";
import { handleExtensions } from "./extensions";
import { registerVuePlugins } from "./vue-plugins";

handleExtensions();

const { exposeClientCommand } = useClient();

exposeClientCommand("loaded", () => {
  const app = createApp(App);
  registerVuePlugins(app);
  app.mount("#app");
});
