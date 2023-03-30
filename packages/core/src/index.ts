import { unmlCtx } from "@unml/kit";
import type { Unml, UnmlHooks } from "@unml/schema";
import type { BrowserWindow, IpcMain, IpcRenderer } from "electron";
import { createHooks } from "hookable";

const initUnml = async (unml: Unml) => {
  unmlCtx.set(unml);
};

export const createNodeUnml = (win: BrowserWindow, ipcMain: IpcMain) => {
  const hooks = createHooks<UnmlHooks>();

  const unml: Unml = {
    callHook: (...args) => {
      win.webContents.send(`unml:callHook-${args[0]}`, ...args);
      return hooks.callHook(...args);
    },
    hook: (...args) => {
      ipcMain.on(`unml:callHook-${args[0]}`, (event, ...args) => {
        hooks.callHook.apply(null, args as any);
      });
      return hooks.hook(...args);
    },
  };

  initUnml(unml);

  return unml;
};

export const createClientUnml = (ipcRenderer: IpcRenderer) => {
  const hooks = createHooks<UnmlHooks>();

  const unml: Unml = {
    callHook: (...args) => {
      ipcRenderer.send(`unml:callHook-${args[0]}`, ...args);
      return hooks.callHook(...args);
    },
    hook: (...args) => {
      ipcRenderer.on(`unml:callHook-${args[0]}`, (event, ...args) => {
        hooks.callHook.apply(null, args as any);
      });
      return hooks.hook(...args);
    },
  };

  initUnml(unml);

  return unml;
};
