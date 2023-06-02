import { ipcMain } from "electron";

import { callNodeCommand } from "@unml/kit";
import { ExtensionLoader } from "@unml/extensions";

let extensionLoader: ExtensionLoader | null = null;

export async function loadExtensions() {
  extensionLoader = new ExtensionLoader();
  await extensionLoader.init();
  await extensionLoader.load();
  await extensionLoader.runLoadEvent();
  ipcMain.handle("command:node:call", (_event, name: string, ...args: any[]) =>
    callNodeCommand(name, ...args),
  );
  await extensionLoader.runRunEvent();
}
