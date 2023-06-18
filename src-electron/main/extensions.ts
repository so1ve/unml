import { COMMAND_NODE_CALL } from "@unml/constants";
import { ExtensionLoader } from "@unml/extensions";
import { callNodeCommand } from "@unml/kit";
import { ipcMain } from "electron";

let extensionLoader: ExtensionLoader | null = null;

export async function loadExtensions() {
  extensionLoader = new ExtensionLoader();
  await extensionLoader.init();
  await extensionLoader.load();
  await extensionLoader.runLoadEvent();
  ipcMain.handle(COMMAND_NODE_CALL, (_event, name: string, ...args: any[]) =>
    callNodeCommand(name, ...args),
  );
  await extensionLoader.runRunEvent();
}
