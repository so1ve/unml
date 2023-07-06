import { COMMAND_CLIENT_CALL, COMMAND_NODE_CALL } from "@unml/constants";
import { ExtensionLoader } from "@unml/extensions";
import { callNodeCommand, exposeNodeCommand } from "@unml/kit";
import { ipcMain } from "electron-better-ipc";

let extensionLoader: ExtensionLoader | null = null;

export async function loadExtensions() {
	extensionLoader = new ExtensionLoader();
	await extensionLoader.init();
	await extensionLoader.load();
	await extensionLoader.runLoadEvent();
	ipcMain.answerRenderer(COMMAND_NODE_CALL, (name: string, ...args: any[]) =>
		callNodeCommand(name, ...args),
	);
	exposeNodeCommand(COMMAND_CLIENT_CALL, (name: string, ...args: any[]) =>
		ipcMain.callFocusedRenderer(COMMAND_CLIENT_CALL, [name, args]),
	);
	await extensionLoader.runRunEvent();
}
