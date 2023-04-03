import type { ServerFunctions } from ".";

export interface ClientFunctions {
  refresh(event: ClientUpdateEvent): void;
  callHook(hook: string, ...args: any[]): Promise<void>;
}

export type ClientUpdateEvent = keyof ServerFunctions;
