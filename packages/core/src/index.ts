import { unmlCtx } from "@unml/kit";
import type { CommandMap, Unml, UnmlHooks } from "@unml/schema";
import { createHooks } from "hookable";

export function createUnml(): Unml {
  const commands: CommandMap = new Map();
  const hooks = createHooks<UnmlHooks>();

  return {
    __commands__: commands,
    hooks,
    hook: hooks.hook,
    callHook: hooks.callHook,
    addHooks: hooks.addHooks,
  } as Unml;
}

export function initUnml(unml: Unml) {
  unmlCtx.set(unml);
}
