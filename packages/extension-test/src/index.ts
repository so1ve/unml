import { addTab, addView, callNodeCommand, exposeNodeCommand } from "@unml/kit";

import type { Activate } from "../../schema/src/extension";

export const activate: Activate = () => ({
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
