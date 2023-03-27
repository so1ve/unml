import type { Unml } from "@unml/schema";
import { getContext } from "unctx";

export const unmlCtx = getContext<Unml>("unml");

export function useUnml (): Unml {
  const instance = unmlCtx.tryUse();
  if (!instance) {
    throw new Error("Unml instance is unavailable!");
  }
  return instance;
}

export function tryUseNuxt (): Unml | null {
  return unmlCtx.tryUse();
}
