import type { MaybePromise } from "@so1ve/utils";
import type { BrowserWindow } from "electron";

export type HookRegister = (ctx: HookRegisterContext) => MaybePromise<void>;

export interface HookRegisterContext {
  win: BrowserWindow;
}
