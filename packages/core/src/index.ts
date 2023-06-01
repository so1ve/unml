import type { MaybePromise } from "@so1ve/utils";
import { unmlCtx } from "@unml/kit";
import type { Unml, UnmlHooks } from "@unml/schema";
import { createHooks } from "hookable";

export function createUnml(): Unml {
  const commands = new Map<string, (...args: any[]) => MaybePromise<void>>();
  const hooks = createHooks<UnmlHooks>();

  return {
    commands,
    hooks,
    hook: hooks.hook,
    callHook: hooks.callHook,
    addHooks: hooks.addHooks,
  };
}

export function initUnml(unml: Unml) {
  unmlCtx.set(unml);
}
