import type { Hookable } from "hookable";

export interface UnmlHooks {
  "window:minimize": () => void;
  "window:maximize": () => void;
  "window:close": () => void;
}

export interface Unml {
  hooks: Hookable<UnmlHooks>;
  hook: Unml["hooks"]["hook"];
  callHook: Unml["hooks"]["callHook"];
  addHooks: Unml["hooks"]["addHooks"];
}
