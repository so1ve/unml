import type { App } from "vue";

import i18n from "./i18n";
import pinia from "./pinia";
import router from "./router";
import vuetify from "./vuetify";

export const registerPlugins = (app: App) => {
  app.use(router).use(pinia).use(i18n).use(vuetify);
};
