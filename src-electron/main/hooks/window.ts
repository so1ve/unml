import type { HookRegister } from "../types";

import { exposeNodeCommand, useUnml } from "@unml/kit";

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

  exposeNodeCommand("window:minimize", minimize);
  exposeNodeCommand("window:maximize", maximize);
  exposeNodeCommand("window:close", close);
};
