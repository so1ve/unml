import { unmlCtx } from "@unml/kit";
import type { Unml, UnmlHooks } from "@unml/schema";
import { createHooks } from "hookable";

export function createUnml(): Unml {
  const hooks = createHooks<UnmlHooks>();

  return {
    hooks,
    hook: hooks.hook,
    callHook: hooks.callHook,
    addHooks: hooks.addHooks,
  };
}

export function initUnml(unml: Unml) {
  unmlCtx.set(unml);
}
