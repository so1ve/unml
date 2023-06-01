import type { Hookable } from "hookable";
import type { MaybePromise } from "@so1ve/utils";

import type { Tab, View } from "./ui";

export interface UnmlHooks {
  "window:minimize": () => void;
  "window:maximize": () => void;
  "window:close": () => void;
  "ui:view": (views: View[]) => void;
  "ui:tabs": (tabs: Tab[]) => void;
}

export interface Unml {
  commands: Map<string, (...args: any[]) => MaybePromise<void>>;
  hooks: Hookable<UnmlHooks>;
  hook: Unml["hooks"]["hook"];
  callHook: Unml["hooks"]["callHook"];
  addHooks: Unml["hooks"]["addHooks"];
}
