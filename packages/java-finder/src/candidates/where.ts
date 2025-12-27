import { existsSync } from "node:fs";

import { execFileText } from "../probe/exec";
import type { JavaSource } from "../types";
import { isWindows } from "../utils/platform";

export async function getCandidatesFromWhere(): Promise<
  { javaPath: string; source: JavaSource }[]
> {
  if (!isWindows()) {
    return [];
  }
  try {
    const out = await execFileText("where", ["java"], { timeoutMs: 5000 });
    const lines = out
      .split(/\r?\n/)
      .map((l) => l.trim())
      .filter(Boolean);
    const results: { javaPath: string; source: JavaSource }[] = [];
    for (const line of lines) {
      if (line.toLowerCase().endsWith("java.exe") && existsSync(line)) {
        results.push({ javaPath: line, source: "where" });
      }
    }

    return results;
  } catch {
    return [];
  }
}
