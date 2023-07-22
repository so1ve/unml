import type { MaybePromise } from "@so1ve/utils";

export type LoadEvent = () => MaybePromise<void>;
export type RunEvent = () => MaybePromise<void>;

export interface Extension {
	/** Do something when the extension is loaded like registering a hook */
	load?: LoadEvent;
	/** Do something when the extension is run like adding a view or calling a hook */
	run?: RunEvent;
}
