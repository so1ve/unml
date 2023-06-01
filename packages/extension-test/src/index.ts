import { addTab, addView, exposeCommand } from "@unml/kit";

import type { Activate } from "../../schema/src/extension";

export const activate: Activate = () => ({
  load: () => {
    exposeCommand("homo114514", () => {
      console.log("114514");

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
      icon: "i-material-symbols:home-rounded", // TODO: fix icon
    });
  },
});
