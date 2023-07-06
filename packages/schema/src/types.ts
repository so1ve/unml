import type { MaybePromise } from "@so1ve/utils";
import type { Hookable } from "hookable";

import type { Tab, View } from "./ui";

export interface Hooks {
  "window:minimize": () => void;
  "window:maximize": () => void;
  "window:close": () => void;
  "ui:views": (views: View[]) => void;
  "ui:tabs": (tabs: Tab[]) => void;
}

export type CommandFn = (...args: any[]) => MaybePromise<any>;
export type CommandMap = Map<string, CommandFn>;

export interface Unml {
  commands: CommandMap;
  hooks: Hookable<Hooks>;
  hook: Unml["hooks"]["hook"];
  callHook: Unml["hooks"]["callHook"];
  addHooks: Unml["hooks"]["addHooks"];
}

export interface Client {
  callNodeCommand: <T = any>(name: string, ...args: any[]) => Promise<T>;
  callClientCommand: <T = any>(name: string, ...args: any[]) => Promise<T>;
  exposeClientCommand: (name: string, fn: CommandFn) => void;
}
