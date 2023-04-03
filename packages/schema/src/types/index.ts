import type { Hookable } from "hookable";

export type HookResult = Promise<void> | void;

type Hooks = Hookable<UnmlHooks>;

export interface UnmlHooks {
  "window:minimize": () => void;
  "window:show": () => void;
  "window:close": () => void;
  "ui:tabs": (tabs: any[]) => void;
  "app:loaded": () => void;
}

export interface Unml {
  hook: Hooks["hook"];
  callHook: Hooks["callHook"];
}

export * from "./rpc";
