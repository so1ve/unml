import { readdir, stat } from "node:fs/promises";
import path from "node:path";

import { resolveJavaFromDir } from "../utils/fs";
import { normalizePathForKey } from "../utils/path";

const DEFAULT_SCAN_KEYWORDS = [
  "java",
  "jdk",
  "jre",
  "jvm",
  "temurin",
  "adoptium",
  "zulu",
  "corretto",
  "openjdk",
  "runtime",
  "minecraft",
  "launcher",
  "mojang",
  "program files",
  "programs",
  "apps",
  "bin",
  "jbr",
];

export async function deepScanForJava(options: {
  roots: string[];
  maxDepth: number;
}): Promise<string[]> {
  const results: string[] = [];
  const visited = new Set<string>();

  interface QueueItem {
    dir: string;
    depth: number;
  }
  const queue: QueueItem[] = options.roots.map((dir) => ({ dir, depth: 0 }));

  while (queue.length > 0) {
    const item = queue.shift()!;
    const dirKey = normalizePathForKey(item.dir);
    if (visited.has(dirKey)) {
      continue;
    }
    visited.add(dirKey);

    if (item.depth > options.maxDepth) {
      continue;
    }

    // quick check
    const { javaPath } = resolveJavaFromDir(item.dir);
    if (javaPath) {
      results.push(javaPath);
      continue;
    }

    let children: string[];
    try {
      children = await readdir(item.dir);
    } catch {
      continue;
    }

    for (const child of children) {
      const full = path.join(item.dir, child);
      let st: any;
      try {
        st = await stat(full);
      } catch {
        continue;
      }
      if (!st.isDirectory()) {
        continue;
      }

      const searchEntry = child.toLowerCase();
      const allow =
        DEFAULT_SCAN_KEYWORDS.some((k) => searchEntry.includes(k)) ||
        /^\d+/.test(searchEntry) ||
        searchEntry === "bin" ||
        item.depth < 1;

      if (!allow) {
        continue;
      }

      queue.push({ dir: full, depth: item.depth + 1 });
    }
  }

  return results;
}
