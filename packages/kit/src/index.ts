import { getContext } from "unctx";
import type { CommandFn, Tab, Unml, View } from "@unml/schema";

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

export function exposeCommand(name: string, fn: CommandFn) {
  const unml = useUnml();
  unml.commands.set(name, fn);
}

export async function callCommand(name: string, ...args: any[]) {
  const unml = useUnml();
  const fn = unml.commands.get(name);
  if (!fn) {
    throw new Error(`Command "${name}" is not exposed!`);
  }

  return await fn(...args);
}
