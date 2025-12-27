import { accessSync, constants as fsConstants } from "node:fs";
import path from "node:path";

import { isWindows } from "./platform";

export function safeAccess(filePath: string): boolean {
  try {
    accessSync(filePath, fsConstants.F_OK);

    return true;
  } catch {
    return false;
  }
}

export function isLikelyJavaBinDir(dir: string): boolean {
  const javaExe = isWindows() ? "java.exe" : "java";
  const javawExe = isWindows() ? "javaw.exe" : "javaw";

  return (
    safeAccess(path.join(dir, javaExe)) || safeAccess(path.join(dir, javawExe))
  );
}

export function resolveJavaFromDir(dir: string): {
  javaPath?: string;
  javawPath?: string;
} {
  const javaExe = isWindows() ? "java.exe" : "java";
  const javawExe = isWindows() ? "javaw.exe" : "javaw";
  const javaPath = path.join(dir, javaExe);
  const javawPath = path.join(dir, javawExe);

  return {
    javaPath: safeAccess(javaPath) ? javaPath : undefined,
    javawPath: safeAccess(javawPath) ? javawPath : undefined,
  };
}

export function guessHomeAndBin(javaPath: string): {
  homeDir: string;
  binDir: string;
} {
  const binDir = path.dirname(javaPath);
  const binBase = path.basename(binDir).toLowerCase();
  if (binBase === "bin") {
    return {
      binDir,
      homeDir: path.dirname(binDir),
    };
  }

  return {
    binDir,
    homeDir: binDir,
  };
}
