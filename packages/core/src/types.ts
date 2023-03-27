import type { Hookable } from "hookable";

export type HookResult = Promise<void> | void;

export type UnmlHooks = {
  "ready": (unml: Unml) => HookResult;
  "close": (unml: Unml) => HookResult;
};

export type Unml = {
  hooks: Hookable<UnmlHooks>;
  hook: Unml["hooks"]["hook"];
  callHook: Unml["hooks"]["callHook"];
  addHooks: Unml["hooks"]["addHooks"];
  ready: () => Promise<void>;
  close: () => Promise<void>;
};
