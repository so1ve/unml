import { App } from "@tinyhttp/app";
import type { Unml } from "@unml/schema";

import { WS_ENTRY, WS_PORT } from "./constants";
import { setupRPC } from "./setup-rpc";
import type { EnhancedRequest } from "./types";

export function startRpcServer (unml: Unml, callback?: () => any) {
  const app = new App<any, EnhancedRequest>();

  const { middleware } = setupRPC(unml);

  app.use(WS_ENTRY, middleware);

  app.listen(WS_PORT, callback);
}

export { WS_ENTRY, WS_PORT } from "./constants";
