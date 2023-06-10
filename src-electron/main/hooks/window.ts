import type { HookRegister } from "../types";

import { exposeNodeCommand } from "@unml/kit";

export const registerHooks: HookRegister = ({ win }) => {
  function minimize() {
    win.minimize();
  }
  function maximize() {
    win.maximize();
  }
  function close() {
    win.close();
  }

  exposeNodeCommand("window:minimize", minimize);
  exposeNodeCommand("window:maximize", maximize);
  exposeNodeCommand("window:close", close);
};
