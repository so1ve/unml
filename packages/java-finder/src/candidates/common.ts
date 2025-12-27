import { existsSync } from "node:fs";
import { readdir, stat } from "node:fs/promises";
import path from "node:path";

import type { JavaSource } from "../types";
import { resolveJavaFromDir } from "../utils/fs";
import { isWindows } from "../utils/platform";

export async function getCandidatesFromCommonLocations(
  env: NodeJS.ProcessEnv,
): Promise<{ javaPath: string; source: JavaSource }[]> {
  const results: { javaPath: string; source: JavaSource }[] = [];

  const roots: string[] = [];
  if (isWindows()) {
    const programFiles = env.ProgramFiles ?? "C:\\Program Files";
    const programFilesX86 =
      env["ProgramFiles(x86)"] ?? "C:\\Program Files (x86)";
    const localAppData = env.LOCALAPPDATA;
    const appData = env.APPDATA;
    const userProfile = env.USERPROFILE;

    roots.push(
      path.join(programFiles, "Java"),
      path.join(programFilesX86, "Java"),
      path.join(programFiles, "Eclipse Adoptium"),
      path.join(programFiles, "Microsoft"),
      path.join(programFiles, "Amazon Corretto"),
      path.join(programFiles, "Zulu"),
    );

    if (userProfile) {
      roots.push(path.join(userProfile, ".jdks"));
    }
    if (appData) {
      roots.push(path.join(appData, ".minecraft", "runtime"));
    }
    if (localAppData) {
      roots.push(path.join(localAppData, "Programs"));
    }
  } else {
    roots.push(
      "/usr/lib/jvm",
      "/usr/java",
      "/Library/Java/JavaVirtualMachines",
    );
  }

  for (const root of roots) {
    if (!root || !existsSync(root)) {
      continue;
    }

    // only 1-level scan; deep scan is separate.
    let entries: string[] = [];
    try {
      entries = await readdir(root);
    } catch {
      continue;
    }

    for (const entry of entries) {
      const full = path.join(root, entry);
      let st: any;
      try {
        st = await stat(full);
      } catch {
        continue;
      }
      if (!st.isDirectory()) {
        continue;
      }

      // Common layouts:
      // - <home>\\bin\\java.exe
      // - <home>\\Contents\\Home\\bin\\java (mac)
      const binDirs = [
        path.join(full, "bin"),
        path.join(full, "jre", "bin"),
        path.join(full, "Contents", "Home", "bin"),
        path.join(full, "Contents", "Home", "jre", "bin"),
      ];

      for (const binDir of binDirs) {
        const { javaPath } = resolveJavaFromDir(binDir);
        if (javaPath) {
          results.push({ javaPath, source: "common" });
        }
      }
    }
  }

  return results;
}
