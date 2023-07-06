import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import {
  addTab,
  addView,
  callNodeCommand,
  defineExtension,
  exposeNodeCommand,
} from "@unml/kit";

const __dirname = dirname(fileURLToPath(import.meta.url));

export default defineExtension({
  load: () => {
    exposeNodeCommand("homo114514", async () => {
      await callNodeCommand("window:minimize");

      return 1_919_810;
    });
  },
  run: async () => {
    addView({
      id: "test",
      path: join(__dirname, "a.html"),
      persistent: true,
    });
    addTab({
      id: "test",
      view: "test",
      icon: {
        type: "iconify",
        icon: "material-symbols:home-rounded",
      },
    });
  },
});
