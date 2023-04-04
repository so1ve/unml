import { unmlCtx, unmlServerCtx } from "@unml/kit";
import type { Server, Unml, UnmlHooks } from "@unml/schema";
import { createHooks } from "hookable";

export const initUnml = (unml: Unml) => {
  unmlCtx.set(unml);
};

export const initServer = (server: Server) => {
  unmlServerCtx.set(server);
};

export function createUnml() {
  const hooks = createHooks<UnmlHooks>();

  const unml: Unml = {
    callHook: hooks.callHook,
    hook: hooks.hook,
  };

  initUnml(unml);

  return unml;
}
