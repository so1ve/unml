import { COMMAND_NODE_CALL } from "@unml/constants";
import { contextBridge, ipcRenderer } from "electron";

contextBridge.exposeInMainWorld("__UNML_API__", {
  callNodeCommand: async (...args: any[]) =>
    ipcRenderer.invoke(COMMAND_NODE_CALL, ...args),
});
