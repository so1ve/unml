import {
  COMMAND_CLIENT_CALL,
  COMMAND_CLIENT_CALL_DONE,
  COMMAND_NODE_CALL,
} from "@unml/constants";
import { ExtensionLoader } from "@unml/extensions";
import { callNodeCommand, exposeNodeCommand } from "@unml/kit";
import { crpr } from "crpr";
import type { BrowserWindow } from "electron";
import { ipcMain } from "electron";

let extensionLoader: ExtensionLoader | null = null;

export async function loadExtensions(win: BrowserWindow) {
  extensionLoader = new ExtensionLoader();
  await extensionLoader.init();
  await extensionLoader.load();
  await extensionLoader.runLoadEvent();
  ipcMain.handle(COMMAND_NODE_CALL, (event, name: string, ...args: any[]) =>
    callNodeCommand(name, ...args),
  );
  // For callClientCommand
  exposeNodeCommand(
    COMMAND_CLIENT_CALL,
    async (name: string, ...args: any[]) => {
      const [promise, resolve, reject] = crpr();
      win.webContents.send(COMMAND_CLIENT_CALL, name, ...args);
      ipcMain.on(
        COMMAND_CLIENT_CALL_DONE,
        function handler(
          event,
          callbackName,
          payload: { failed: boolean; result: any },
        ) {
          if (callbackName === name) {
            ipcMain.off(COMMAND_CLIENT_CALL_DONE, handler);
            if (payload.failed) {
              reject(payload.result);
            } else {
              resolve(payload.result);
            }
          }
        },
      );

      return promise;
    },
  );
  await extensionLoader.runRunEvent();
}
