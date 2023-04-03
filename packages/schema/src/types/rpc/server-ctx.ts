import type { BirpcGroup } from "birpc";

import type { Unml } from "..";

import type { ClientFunctions, ServerFunctions } from ".";

/**
 * @internal
 */
export interface UnmlServerContext {
  unml: Unml;
  rpc: BirpcGroup<ClientFunctions, ServerFunctions>;

  /**
   * Invalidate client cache for a function and ask for re-fetching
   */
  refresh: (event: keyof ServerFunctions) => void;

  extendServerRpc: <ClientFunctions = {}, ServerFunctions = {}>(
    name: string,
    functions: ServerFunctions,
  ) => BirpcGroup<ClientFunctions, ServerFunctions>;
}
