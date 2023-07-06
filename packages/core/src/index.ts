import { unmlCtx } from "@unml/kit";
import type { CommandMap, Hooks, Unml } from "@unml/schema";
import { createHooks } from "hookable";

export function createUnml(): Unml {
  const commands: CommandMap = new Map();
  const hooks = createHooks<Hooks>();

  return {
    commands,
    hooks,
    hook: hooks.hook,
    callHook: hooks.callHook,
    addHooks: hooks.addHooks,
  } as Unml;
}

export function initUnml(unml: Unml) {
  unmlCtx.set(unml);
}
