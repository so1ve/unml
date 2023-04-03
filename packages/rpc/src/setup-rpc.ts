// Adapted from nuxt/devtools
import type { ClientFunctions, ServerFunctions, Unml, UnmlServerContext } from "@unml/schema";
import { createBirpcGroup } from "birpc";
import type { ChannelOptions } from "birpc";
import { parse, stringify } from "flatted";
import type { WebSocket } from "ws";

import { setupCustomTabRPC } from "./rpcs/custom-tabs";
import type { EnhancedRequest } from "./types";

// import type { ClientFunctions, ModuleOptions, NuxtDevtoolsServerContext, ServerFunctions } from "../types";

// import { setupAssetsRPC } from "./assets";
// import { setupCustomTabRPC } from "./custom-tabs";
// import { setupGeneralRPC } from "./general";
// import { setupNpmRPC } from "./npm";
// import { setupStorageRPC } from "./storage";
// import { setupTerminalRPC } from "./terminals";
// import { setupWizardRPC } from "./wizard";

export function setupRPC (unml: Unml) {
  const serverFunctions = {} as ServerFunctions;
  const extendedRpcMap = new Map<string, any>();
  const rpc = createBirpcGroup<ClientFunctions, ServerFunctions>(
    serverFunctions,
    [],
    {
      resolver: (name, fn) => {
        if (fn) {
          return fn;
        }

        if (!name.includes(":")) {
          return;
        }

        const [namespace, fnName] = name.split(":");
        return extendedRpcMap.get(namespace)?.[fnName];
      },
      onError (error, name) {
        console.error(`[UNML RPC] RPC error on executing "${name}":`, error);
      },
    },
  );

  function refresh (event: keyof ServerFunctions) {
    rpc.broadcast.refresh.asEvent(event);
  }

  function extendServerRpc (namespace: string, functions: any): any {
    extendedRpcMap.set(namespace, functions);

    return {
      broadcast: new Proxy({}, {
        get: (_, key) => {
          if (typeof key !== "string") {
            return;
          }
          return (rpc.broadcast as any)[`${namespace}:${key}`];
        },
      }),
    };
  }

  const ctx: UnmlServerContext = {
    unml,
    rpc,
    refresh,
    extendServerRpc,
  };

  Object.assign(
    serverFunctions,
    {
      ...setupCustomTabRPC(ctx),
      // TODO
    } as any satisfies ServerFunctions,
  );

  const wsClients = new Set<WebSocket>();
  const middleware = async (
    req: EnhancedRequest,
    res: any,
    next: () => void | Promise<void>,
  ) => {
    // Handle WebSocket
    if (req.ws) {
      const ws = await req.ws();
      wsClients.add(ws);
      const channel: ChannelOptions = {
        post: d => ws.send(d),
        on: fn => ws.on("message", fn),
        serialize: stringify,
        deserialize: parse,
      };
      rpc.updateChannels((c) => {
        c.push(channel);
      });
      ws.on("close", () => {
        wsClients.delete(ws);
        rpc.updateChannels((c) => {
          const index = c.indexOf(channel);
          if (index >= 0) {
            c.splice(index, 1);
          }
        });
      });
    } else {
      next();
    }
  };

  return {
    middleware,
    ...ctx,
  };
}
