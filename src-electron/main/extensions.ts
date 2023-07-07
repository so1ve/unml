import { mkdir } from "node:fs/promises";
import { join } from "node:path";

import { COMMAND_CLIENT_CALL, COMMAND_NODE_CALL } from "@unml/constants";
import { ExtensionLoader } from "@unml/extensions";
import { callNodeCommand, exposeNodeCommand } from "@unml/kit";
import { exists } from "@unml/utils";
import { ipcMain } from "electron-better-ipc";

let extensionLoader: ExtensionLoader | null = null;

async function ensureNodeModules() {
  if (!(await exists(join(process.cwd(), "node_modules")))) {
    await mkdir(join(process.cwd(), "node_modules"));
  }
}

export async function loadExtensions() {
  await ensureNodeModules();
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
