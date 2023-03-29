import { createApp } from "vue";
import "uno.css";
import "@unocss/reset/tailwind.css";

import "./styles/main.scss";
import App from "./App.vue";
import { registerPlugins } from "./plugins";

const app = createApp(App);
registerPlugins(app);
app.mount("#app");

const unml = useUnml();

console.log(unml);
