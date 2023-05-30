import { createBirpc } from "birpc";
import { parse, stringify } from "flatted";

import type { ClientFunctions, ServerFunctions } from "@unml/schema";

// Copy again to avoid import the @tinyhttp/app package.
export const WS_PORT = 11_451; // Homo Port
export const WS_ENTRY = "/ws";
const RECONNECT_INTERVAL = 2000;

export const wsConnecting = ref(true);
export const wsError = ref();

let connectPromise = connectWS();
let onMessage: Function = () => {};

export const clientFunctions = {
  // will be added in app.vue
} as ClientFunctions;

export const extendedRpcMap = new Map<string, any>();

export const rpc = createBirpc<ServerFunctions>(clientFunctions, {
  post: async (d) => {
    (await connectPromise).send(d);
  },
  on: (fn) => {
    onMessage = fn;
  },
  serialize: stringify,
  deserialize: parse,
  resolver(name, fn) {
    if (fn) {
      return fn;
    }
    if (!name.includes(":")) {
      return;
    }
    const [namespace, fnName] = name.split(":");

    return extendedRpcMap.get(namespace)?.[fnName];
  },
  onError(error, name) {
    console.error(`[UNML RPC] RPC error on executing "${name}":`, error);
  },
});

async function connectWS() {
  const wsUrl = new URL("ws://host");
  wsUrl.protocol = location.protocol === "https:" ? "wss:" : "ws:";
  wsUrl.pathname = WS_ENTRY;
  wsUrl.host = location.host;
  wsUrl.port = String(WS_PORT);

  const ws = new WebSocket(wsUrl.toString());
  ws.addEventListener("message", (e) => onMessage(String(e.data)));
  ws.addEventListener("error", (e) => {
    console.error(e);
    wsError.value = e;
  });
  ws.addEventListener("close", () => {
    // eslint-disable-next-line no-console
    console.log("[UNML RPC] WebSocket closed, reconnecting...");
    wsConnecting.value = true;
    setTimeout(async () => {
      connectPromise = connectWS();
    }, RECONNECT_INTERVAL);
  });
  wsConnecting.value = true;
  if (ws.readyState !== WebSocket.OPEN) {
    await new Promise((resolve) => ws.addEventListener("open", resolve));
  }

  // eslint-disable-next-line no-console
  console.log("[UNML RPC] WebSocket connected.");
  wsConnecting.value = false;
  wsError.value = null;

  return ws;
}
