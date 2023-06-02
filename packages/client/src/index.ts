import type { UnmlClient } from "@unml/schema";
import { crpr } from "crpr";

export function useClient(): UnmlClient {
  if (window.self === window.top) {
    const client: UnmlClient = {
      callNodeCommand: async (...args) => {
        const { ipcRenderer } = await import("electron");

        return ipcRenderer.invoke("command:node:call", ...args);
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
      window.parent.postMessage({ name: "command:node:call", args }, "*");

      const [promise, resolve] = crpr<any>();

      window.addEventListener("message", function handler(event) {
        if (event.data.name === "command:node:call:done") {
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