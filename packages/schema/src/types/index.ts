import type { Hookable } from "hookable";

import type { CustomTab, UnmlServerContext } from "./rpc";

export type HookResult = Promise<void> | void;

type Hooks = Hookable<UnmlHooks>;

export interface UnmlHooks {
  "window:minimize": () => void;
  "window:show": () => void;
  "window:close": () => void;
  "ui:tabs": (tabs: CustomTab[]) => void;
  "app:loaded": () => void;
}

export interface Unml {
  hook: Hooks["hook"];
  callHook: Hooks["callHook"];
}

export type Server = UnmlServerContext;

export * from "./rpc";
