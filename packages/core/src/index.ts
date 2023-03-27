import { unmlCtx } from "@unml/kit";
import { createHooks } from "hookable";

import type { Unml, UnmlHooks } from "./types";

const initUnml = async (unml: Unml) => {
  unmlCtx.set(unml);
  unml.hook("close", () => unmlCtx.unset());
};

export const createUnml = () => {
  const hooks = createHooks<UnmlHooks>();

  const unml: Unml = {
    hooks,
    callHook: hooks.callHook,
    addHooks: hooks.addHooks,
    hook: hooks.hook,
    ready: () => initUnml(unml),
    close: () => Promise.resolve(hooks.callHook("close", unml)),
  };

  return unml;
};

export type { HookResult, Unml, UnmlHooks } from "./types";
