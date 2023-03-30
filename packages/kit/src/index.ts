import type { Unml } from "@unml/schema";
import { getContext } from "unctx";

const CTX_KEY = "unml";

export const unmlCtx = getContext<Unml>(CTX_KEY);

export const useUnml = () => {
  const instance = unmlCtx.tryUse();
  if (!instance) { throw new Error("Unml instance is unavailable!"); }
  return instance;
};

export const tryUseUnml = () => unmlCtx.tryUse();
