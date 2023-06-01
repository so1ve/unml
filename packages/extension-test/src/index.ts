import { addView, exposeCommand } from "@unml/kit";

import type { Activate } from "../../schema/src/extension";

export const activate: Activate = () => ({
  load: () => {
    exposeCommand("homo114514", () => {
      console.log("114514");
    });
  },
  run: () => {
    addView({
      id: "test",
      path: "114",
    });
  },
});
