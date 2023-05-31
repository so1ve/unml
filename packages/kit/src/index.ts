import { getContext } from "unctx";
import type { Tab, Unml, View } from "@unml/schema";

export const unmlCtx = getContext<Unml>("unml");

export function useUnml() {
  const instance = unmlCtx.tryUse();
  if (!instance) {
    throw new Error("UNML instance is unavailable!");
  }

  return instance;
}

export const tryUseUnml = unmlCtx.tryUse;

export function addView(view: View) {
  const unml = useUnml();
  unml.hook("ui:view", (views) => {
    views.push(view);
  });
}

export function addTab(tab: Tab) {
  const unml = useUnml();
  unml.hook("ui:tabs", (tabs) => {
    tabs.push(tab);
  });
}
