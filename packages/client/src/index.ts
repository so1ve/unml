import { COMMAND_NODE_CALL, COMMAND_NODE_CALL_DONE } from "@unml/constants";
import type { UnmlClient } from "@unml/schema";
import { crpr } from "crpr";

export function useClient(): UnmlClient {
  if (window.self === window.top) {
    const client: UnmlClient = {
      callNodeCommand: async (...args) => {
        const { ipcRenderer } = await import("electron");

        return ipcRenderer.invoke(COMMAND_NODE_CALL, ...args);
      },

      // TODO
      exposeClientCommand: () => {},
    };

    return client;
  }
  // In an iframe
  // TODO
  const client: UnmlClient = {
    callNodeCommand: (...args) => {
      window.parent.postMessage({ name: COMMAND_NODE_CALL, args }, "*");

      const [promise, resolve] = crpr<any>();

      window.addEventListener("message", function handler(event) {
        if (event.data.name === COMMAND_NODE_CALL_DONE) {
          window.removeEventListener("message", handler);
          resolve(event.data.result);
        }
      });

      return promise;
    },
    exposeClientCommand: () => {},
  };

  return client;
}
