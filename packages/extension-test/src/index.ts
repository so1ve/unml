import { addView } from "@unml/kit";

import type { Activate } from "../../schema/src/extension";

export const activate: Activate = () => ({
  load: () => {},
  run: () => {
    addView({
      id: "test",
      path: "114",
    });
  },
});
