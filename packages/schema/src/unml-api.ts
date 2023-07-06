import type { MaybePromise } from "@so1ve/utils";

export interface UnmlApi {
	callNodeCommand: (...args: any[]) => MaybePromise<any>;
	onCallClientCommand: (
		handler: (name: string, ...args: any[]) => MaybePromise<any>,
	) => void;
	process: NodeJS.Process;
}
