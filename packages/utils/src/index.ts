import { access } from "node:fs/promises";
import os from "node:os";

import { normalize } from "pathe";
import { withTrailingSlash } from "ufo";

export const exists = (d: string) =>
	access(d).then(
		() => true,
		() => false,
	);

export function normalizePath(path: string) {
	if (os.platform() !== "win32") {
		return path;
	}
	path = normalize(path);
	const splitted = path.split("/");
	const drive = splitted[0].toUpperCase();
	const rest = splitted.slice(1).join("/");

	return `${drive}:/${rest}`;
}

export function isParentDirectory(dir: string, file: string): boolean {
	dir = withTrailingSlash(dir);

	// TODO: case sensitive filesystem
	return file.startsWith(dir);
}
