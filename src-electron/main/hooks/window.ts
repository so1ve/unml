import type { HookRegister } from "../types";

import { exposeCommand, useUnml } from "@unml/kit";

export const registerHooks: HookRegister = ({ win }) => {
  const unml = useUnml();

  function minimize() {
    win.minimize();
  }
  function maximize() {
    win.maximize();
  }
  function close() {
    win.close();
  }

  unml.addHooks({
    "window:minimize": minimize,
    "window:maximize": maximize,
    "window:close": close,
  });

  exposeCommand("window:minimize", minimize);
  exposeCommand("window:maximize", maximize);
  exposeCommand("window:close", close);
};
