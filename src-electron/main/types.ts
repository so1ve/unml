import type { MaybePromise } from "@so1ve/utils";
import type { BrowserWindow } from "electron";

export type CommandRegister = (
	ctx: CommandRegisterContext,
) => MaybePromise<void>;

export interface CommandRegisterContext {
	win: BrowserWindow;
}
