import {
  addTab,
  addView,
  callNodeCommand,
  defineExtension,
  exposeNodeCommand,
} from "@unml/kit";

export default defineExtension({
  load: () => {
    exposeNodeCommand("homo114514", async () => {
      await callNodeCommand("window:minimize");

      return 1_919_810;
    });
  },
  run: () => {
    addView({
      id: "test",
      path: "114",
    });
    addTab({
      id: "test",
      view: "test",
      icon: "material-symbols:home-rounded",
    });
  },
});
