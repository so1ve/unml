import path from "node:path";

import type { JavaSource } from "../types";
import { isLikelyJavaBinDir, resolveJavaFromDir } from "../utils/fs";
import { toWindowsPath } from "../utils/path";
import { isWindows } from "../utils/platform";

export async function getCandidatesFromEnv(
  env: NodeJS.ProcessEnv,
): Promise<{ javaPath: string; source: JavaSource }[]> {
  const results: { javaPath: string; source: JavaSource }[] = [];

  const javaHome = env.JAVA_HOME?.trim();
  if (javaHome) {
    const home = isWindows() ? toWindowsPath(javaHome) : javaHome;
    const dirs = [path.join(home, "bin"), home];
    for (const dir of dirs) {
      const { javaPath } = resolveJavaFromDir(dir);
      if (javaPath) {
        results.push({ javaPath, source: "env:JAVA_HOME" });
      }
    }
  }

  const pathEnv = env.Path ?? env.PATH ?? "";
  const segments = pathEnv
    .split(isWindows() ? ";" : ":")
    .map((s) => s.trim().replaceAll('"', ""))
    .filter(Boolean);

  for (const seg of segments) {
    const dir = isWindows() ? toWindowsPath(seg) : seg;
    if (!isLikelyJavaBinDir(dir)) {
      continue;
    }
    const { javaPath } = resolveJavaFromDir(dir);
    if (javaPath) {
      results.push({ javaPath, source: "env:PATH" });
    }
  }

  return results;
}
