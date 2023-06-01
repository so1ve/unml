import type { UnmlClient } from "@unml/schema";

export function useClient(): UnmlClient {
  if (window.self === window.top) {
    throw new Error("useClient must be used in UNML iframe");
  }
  const client: UnmlClient = {
    callCommand: (...args) => {
      window.parent.postMessage({ name: "command:call", args }, "*");

      return new Promise((resolve) => {
        window.addEventListener("message", function handler(event) {
          if (event.data.name === "command:call:done") {
            window.removeEventListener("message", handler);
            resolve(event.data.result);
          }
        });
      });
    },
  };

  return client;
}
