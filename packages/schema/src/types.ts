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

export type CommandFn = (...args: any[]) => MaybePromise<any>;
export type CommandMap = Map<string, CommandFn>;

export interface Unml {
  hooks: Hookable<UnmlHooks>;
  hook: Unml["hooks"]["hook"];
  callHook: Unml["hooks"]["callHook"];
  addHooks: Unml["hooks"]["addHooks"];
}

export interface UnmlInternal extends Unml {
  /** @internal */
  __commands__: CommandMap;
}

export interface UnmlClient {
  callNodeCommand: <T = any>(name: string, ...args: any[]) => Promise<T>;
  // TODO: add callClientCommand
  exposeClientCommand: (name: string, fn: CommandFn) => void;
}
