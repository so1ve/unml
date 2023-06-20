import type { MaybePromise } from "@so1ve/utils";
import {
  API_VAR,
  COMMAND_CLIENT_CALL,
  COMMAND_CLIENT_CALL_DONE,
  COMMAND_NODE_CALL,
} from "@unml/constants";
import type { UnmlApi } from "@unml/schema";
import { contextBridge, ipcRenderer } from "electron";

contextBridge.exposeInMainWorld(API_VAR, {
  callNodeCommand: async (...args: any[]) =>
    ipcRenderer.invoke(COMMAND_NODE_CALL, ...args),
  onCallClientCommand: (
    handler: (name: string, ...args: any[]) => MaybePromise<any>,
  ) => {
    ipcRenderer.on(
      COMMAND_CLIENT_CALL,
      async (event, name: string, ...args) => {
        let failed = false;
        let result: any;
        try {
          result = await handler(name, ...args);
        } catch (e) {
          failed = true;
          result = e;
        }
        ipcRenderer.send(COMMAND_CLIENT_CALL_DONE, name, {
          failed,
          result,
        });
      },
    );
  },
} as UnmlApi);
