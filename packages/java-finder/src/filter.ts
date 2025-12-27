import os from "node:os";

import type { FindJavaFilter, JavaProbeResult } from "./types";
import { normalizePathForKey } from "./utils/path";

export function shouldKeepCandidate(
  candidate: JavaProbeResult,
  filter?: FindJavaFilter,
): boolean {
  if (!filter) {
    return true;
  }

  if (filter.excludeMinecraftRuntime) {
    const key = normalizePathForKey(candidate.homeDir);
    if (key.includes("\\\\.minecraft\\\\runtime\\\\")) {
      return false;
    }
  }

  if (
    filter.minMajor !== undefined &&
    candidate.version.major < filter.minMajor
  ) {
    return false;
  }
  if (
    filter.maxMajor !== undefined &&
    candidate.version.major > filter.maxMajor
  ) {
    return false;
  }

  if (
    filter.disallowHighJre &&
    candidate.isJre &&
    candidate.version.major >= 16
  ) {
    return false;
  }

  if (filter.require64BitOn64BitOS) {
    const is64Os = os.arch() === "x64" || os.arch() === "arm64";
    if (is64Os && candidate.is64Bit === false) {
      return false;
    }
  }

  return true;
}
