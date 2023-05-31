import { getContext } from "unctx";
import type { Unml } from "@unml/schema";

export const unmlCtx = getContext<Unml>("unml");

export function useUnml() {
  const instance = unmlCtx.tryUse();
  if (!instance) {
    throw new Error("UNML instance is unavailable!");
  }

  return instance;
}

export const tryUseUnml = unmlCtx.tryUse;
