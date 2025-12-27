import { existsSync } from "node:fs";
import path from "node:path";

import { DEFAULT_PROBE_TIMEOUT_MS } from "../constants";
import type { JavaProbeResult } from "../types";
import { guessHomeAndBin, resolveJavaFromDir, safeAccess } from "../utils/fs";
import { isWindows } from "../utils/platform";
import { execFileText } from "./exec";
import {
  detectVendor,
  parseArchFromOutput,
  parseJavaVersionFromOutput,
} from "./parse";

export async function probeJava(
  javaPath: string,
  options?: { timeoutMs?: number },
): Promise<JavaProbeResult> {
  const timeoutMs = options?.timeoutMs ?? DEFAULT_PROBE_TIMEOUT_MS;
  if (!existsSync(javaPath)) {
    throw new Error(`java not found: ${javaPath}`);
  }

  const { homeDir, binDir } = guessHomeAndBin(javaPath);
  const { javawPath } = resolveJavaFromDir(binDir);

  // Prefer a richer probe; some Javas print most info to stderr.
  let output = await execFileText(
    javaPath,
    ["-XshowSettings:properties", "-version"],
    {
      timeoutMs,
    },
  );

  // Fallback
  if (!output.trim()) {
    output = await execFileText(javaPath, ["-version"], { timeoutMs });
  }

  const version = parseJavaVersionFromOutput(output);
  if (!version.major) {
    throw new Error(`failed to parse java version from output: ${javaPath}`);
  }

  const { arch, is64Bit } = parseArchFromOutput(output);

  const javacExe = isWindows() ? "javac.exe" : "javac";
  const isJre = !safeAccess(path.join(binDir, javacExe));

  return {
    javaPath,
    javawPath,
    homeDir,
    binDir,
    isJre,
    is64Bit,
    arch,
    version,
    vendor: detectVendor(output),
    output,
  };
}
