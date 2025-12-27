import path from "node:path";

import { execFileText } from "../probe/exec";
import type { JavaSource } from "../types";
import { resolveJavaFromDir } from "../utils/fs";
import { toWindowsPath } from "../utils/path";
import { isWindows } from "../utils/platform";

const regQueryAll = async (key: string): Promise<string> =>
  execFileText("reg", ["query", key, "/s"], { timeoutMs: 8000 });

function parseRegHomes(output: string): string[] {
  const homes: string[] = [];
  const lines = output.split(/\r?\n/);
  for (const line of lines) {
    // e.g. "    JavaHome    REG_SZ    C:\\Program Files\\Java\\jdk-17"
    const match = /^\s*([\w().-]+)\s+REG_\w+\s+(\S.*)$/i.exec(line);
    if (!match) {
      continue;
    }
    const name = match[1].trim();
    const value = match[2].trim();
    if (!value) {
      continue;
    }

    const keyName = name.toLowerCase();
    if (
      keyName === "javahome" ||
      keyName === "installationpath" ||
      keyName === "path"
    ) {
      homes.push(value);
    }
  }

  return homes;
}

export async function getCandidatesFromRegistry(): Promise<
  { javaPath: string; source: JavaSource }[]
> {
  if (!isWindows()) {
    return [];
  }

  const keys = [
    // JavaSoft
    "HKLM\\SOFTWARE\\JavaSoft\\Java Runtime Environment",
    "HKLM\\SOFTWARE\\JavaSoft\\Java Development Kit",
    "HKLM\\SOFTWARE\\WOW6432Node\\JavaSoft\\Java Runtime Environment",
    "HKLM\\SOFTWARE\\WOW6432Node\\JavaSoft\\Java Development Kit",
    "HKCU\\SOFTWARE\\JavaSoft\\Java Runtime Environment",
    "HKCU\\SOFTWARE\\JavaSoft\\Java Development Kit",

    // Adoptium/Temurin
    "HKLM\\SOFTWARE\\Eclipse Adoptium\\JDK",
    "HKLM\\SOFTWARE\\Eclipse Adoptium\\JRE",
    "HKLM\\SOFTWARE\\WOW6432Node\\Eclipse Adoptium\\JDK",
    "HKLM\\SOFTWARE\\WOW6432Node\\Eclipse Adoptium\\JRE",

    // Microsoft Build of OpenJDK
    "HKLM\\SOFTWARE\\Microsoft\\JDK",
    "HKLM\\SOFTWARE\\WOW6432Node\\Microsoft\\JDK",
  ];

  const homeDirs: string[] = [];
  for (const key of keys) {
    try {
      const out = await regQueryAll(key);
      homeDirs.push(...parseRegHomes(out));
    } catch {
      // ignore missing keys
    }
  }

  const results: { javaPath: string; source: JavaSource }[] = [];
  for (const home of homeDirs) {
    const homeNorm = toWindowsPath(home.trim().replaceAll('"', ""));
    const candidates = [path.join(homeNorm, "bin"), homeNorm];
    for (const dir of candidates) {
      const { javaPath } = resolveJavaFromDir(dir);
      if (javaPath) {
        results.push({ javaPath, source: "registry" });
      }
    }
  }

  return results;
}
