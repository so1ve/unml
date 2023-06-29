import type { MaybePromise } from "@so1ve/utils";
import {
  API_VAR,
  COMMAND_CLIENT_CALL,
  COMMAND_NODE_CALL,
} from "@unml/constants";
import type { UnmlApi } from "@unml/schema";
import { contextBridge } from "electron";
import { ipcRenderer } from "electron-better-ipc";

contextBridge.exposeInMainWorld(API_VAR, {
  callNodeCommand: async (...args: any[]) =>
    ipcRenderer.callMain(COMMAND_NODE_CALL, ...args),
  onCallClientCommand: (
    handler: (name: string, ...args: any[]) => MaybePromise<any>,
  ) => {
    ipcRenderer.answerMain(
      COMMAND_CLIENT_CALL,
      ([name, args]: [string, any[]]) => handler(name, ...args),
    );
  },
} as UnmlApi);
