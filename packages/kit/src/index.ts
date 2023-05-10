import type { Server, Unml } from "@unml/schema";
import { getContext } from "unctx";

const CTX_KEY = "unml";
const SERVER_CTX_KEY = "unml-server";

export const unmlCtx = getContext<Unml>(CTX_KEY);
export const unmlServerCtx = getContext<Server>(SERVER_CTX_KEY);

export function useUnml() {
  const instance = unmlCtx.tryUse();
  if (!instance) {
    throw new Error("Unml instance is unavailable!");
  }

  return instance;
}
export const tryUseUnml = () => unmlCtx.tryUse();

// TODO: Integrate this into useUnml
export function useUnmlServer() {
  const instance = unmlServerCtx.tryUse();
  if (!instance) {
    throw new Error("Unml server instance is unavailable!");
  }

  return instance;
}
export const tryUseUnmlServer = () => unmlServerCtx.tryUse();
