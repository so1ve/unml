import { App } from "@tinyhttp/app";
import type { Unml, UnmlServerContext } from "@unml/schema";
import { tinyws } from "tinyws";

import { WS_ENTRY, WS_PORT } from "./constants";
import { setupRPC } from "./setup-rpc";
import type { EnhancedRequest } from "./types";

export function startRpcServer(
  unml: Unml,
  callback?: (ctx: UnmlServerContext) => any,
) {
  const app = new App<any, EnhancedRequest>();

  const { middleware, ...ctx } = setupRPC(unml);

  app.use(tinyws());
  app.use(WS_ENTRY, middleware);

  app.listen(WS_PORT, () => callback?.(ctx));
}

export { WS_ENTRY, WS_PORT } from "./constants";
