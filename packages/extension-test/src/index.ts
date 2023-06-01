import { addTab, addView, callCommand, exposeCommand } from "@unml/kit";

import type { Activate } from "../../schema/src/extension";

export const activate: Activate = () => ({
  load: () => {
    exposeCommand("homo114514", async () => {
      await callCommand("window:minimize");

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
