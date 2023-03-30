import fsp from "node:fs/promises";
import path from "node:path";

export const loadPackagesFromCwd = (cwd = process.cwd()) => fsp.readdir(path.join(cwd, "node_modules"));
