import type { MaybePromise } from "@so1ve/utils";

export interface Extension {
  activate: () => MaybePromise<void>;
}
