import { API_VAR, COMMAND_NODE_CALL } from "@unml/constants";
import type { CommandMap, UnmlClient } from "@unml/schema";
import { crpr } from "crpr";

// TODO
const commands: CommandMap = new Map();

export function useClient(): UnmlClient {
	if (window.self === window.top) {
		const client: UnmlClient = {
			callNodeCommand: async (...args) =>
				window[API_VAR].callNodeCommand(...args),

			callClientCommand: async (name, ...args) => {
				if (!commands.has(name)) {
					throw new Error(`Command "${name}" is not exposed!`);
				}

				return commands.get(name)!(...args);
			},

			exposeClientCommand: (name, fn) => {
				if (commands.has(name)) {
					throw new Error(`Command "${name}" is already exposed!`);
				}
				commands.set(name, fn);
			},
		};

		return client;
	}
	// In an iframe
	// TODO
	const client: UnmlClient = {
		callNodeCommand: (...args) => {
			window.parent.postMessage({ name: COMMAND_NODE_CALL, args }, "*");

			const [promise, _resolve] = crpr<any>();

			// window.addEventListener("message", function handler(event) {
			//   if (event.data.name === COMMAND_NODE_CALL_DONE) {
			//     window.removeEventListener("message", handler);
			//     resolve(event.data.result);
			//   }
			// });

			return promise;
		},

		// TODO
		callClientCommand: async () => undefined as any,
		exposeClientCommand: () => {},
	};

	return client;
}
