import { API_VAR, RESOURCE_PROTOCOL } from "@unml/constants";
import { normalize } from "pathe";

export const process = globalThis.process ?? window[API_VAR].process;

export function pathToResourceUrl(path: string) {
	const isWindows = process.platform === "win32";
	if (isWindows) {
		path = normalize(path);
		const splitted = path.split("/");
		if (splitted[0].endsWith(":")) {
			splitted[0] = splitted[0].slice(0, -1);
		}
		path = splitted.join("/");
	}

	return `${RESOURCE_PROTOCOL}://${path}`;
}
