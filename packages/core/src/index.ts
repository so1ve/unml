import { unmlCtx } from "@unml/kit";
import type { Unml, UnmlHooks } from "@unml/schema";
import { createHooks } from "hookable";

const initUnml = async (unml: Unml) => {
  unmlCtx.set(unml);
};

export const createUnml = () => {
  const hooks = createHooks<UnmlHooks>();

  const unml: Unml = {
    hooks,
    callHook: hooks.callHook,
    addHooks: hooks.addHooks,
    hook: hooks.hook,
  };

  initUnml(unml);

  return unml;
};
