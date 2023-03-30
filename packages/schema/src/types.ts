import type { Hookable } from "hookable";

type Hooks = Hookable<UnmlHooks>;

export type HookResult = Promise<void> | void;

export interface UnmlHooks {
  "window:minimize": () => void;
  "window:show": () => void;
  "window:close": () => void;
}

export interface Unml {
  hook: Hooks["hook"];
  callHook: Hooks["callHook"];
}
