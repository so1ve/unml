import path from "node:path";

import { isWindows } from "./platform";

export function normalizePathForKey(p: string): string {
  if (isWindows()) {
    return path.win32.normalize(p).toLowerCase();
  }

  return path.posix.normalize(p);
}

export const toWindowsPath = (p: string): string => p.replaceAll("/", "\\");
