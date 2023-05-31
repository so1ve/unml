import type { HookRegister } from "../types";

import { useUnml } from "@unml/kit";

export const registerHooks: HookRegister = ({ win }) => {
  const unml = useUnml();
  unml.addHooks({
    "window:minimize": () => {
      win.minimize();
    },
    "window:maximize": () => {
      win.maximize();
    },
    "window:close": () => {
      win.close();
    },
  });
};
