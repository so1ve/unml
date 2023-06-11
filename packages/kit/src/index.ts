import { getContext } from "unctx";
import type { CommandFn, Tab, Unml, UnmlInternal, View } from "@unml/schema";

export * from "./helpers";

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

export function exposeNodeCommand(name: string, fn: CommandFn) {
  const unml = useUnml() as UnmlInternal;
  unml.__commands__.set(name, fn);
}

export async function callNodeCommand(name: string, ...args: any[]) {
  const unml = useUnml() as UnmlInternal;
  const fn = unml.__commands__.get(name);
  if (!fn) {
    throw new Error(`Command "${name}" is not exposed!`);
  }

  return await fn(...args);
}

// TODO: add callClientCommand
export function callClientCommand() {}
