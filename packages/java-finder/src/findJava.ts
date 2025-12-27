import { getCandidatesFromCommonLocations } from "./candidates/common";
import { getCandidatesFromEnv } from "./candidates/env";
import { getCandidatesFromWhere } from "./candidates/where";
import { DEFAULT_MAX_PROBE_COUNT, DEFAULT_PROBE_TIMEOUT_MS } from "./constants";
import { shouldKeepCandidate } from "./filter";
import { probeJava } from "./probe/probeJava";
import { deepScanForJava } from "./scan/deepScan";
import { sortJavaCandidates } from "./sort";
import type { FindJavaOptions, JavaCandidate, JavaSource } from "./types";
import { normalizePathForKey } from "./utils/path";
import { isWindows } from "./utils/platform";
import { getCandidatesFromRegistry } from "./windows/registry";

export async function findJava(
  options?: FindJavaOptions,
): Promise<JavaCandidate[]> {
  const env = options?.env ?? process.env;
  const probeTimeoutMs = options?.probeTimeoutMs ?? DEFAULT_PROBE_TIMEOUT_MS;
  const maxProbeCount = options?.maxProbeCount ?? DEFAULT_MAX_PROBE_COUNT;

  const includeEnv = options?.includeEnv ?? true;
  const includeRegistry = options?.includeRegistry ?? true;
  const includeWhere = options?.includeWhere ?? true;
  const includeCommonLocations = options?.includeCommonLocations ?? true;

  const filter = options?.filter;

  const discovered: { javaPath: string; source: JavaSource }[] = [];

  if (includeEnv) {
    discovered.push(...(await getCandidatesFromEnv(env)));
  }
  if (includeWhere) {
    discovered.push(...(await getCandidatesFromWhere()));
  }
  if (includeRegistry) {
    discovered.push(...(await getCandidatesFromRegistry()));
  }
  if (includeCommonLocations) {
    discovered.push(...(await getCandidatesFromCommonLocations(env)));
  }

  if (options?.deepScan) {
    const maxDepth = options.deepScanMaxDepth ?? 5;
    let roots = options.deepScanRoots;
    if (!roots?.length) {
      if (isWindows()) {
        // Best-effort: default to common drive letters.
        roots = ["C:\\", "D:\\", "E:\\"];
      } else {
        roots = ["/"];
      }
    }

    const scanJavaPaths = await deepScanForJava({ roots, maxDepth });
    discovered.push(
      ...scanJavaPaths.map((javaPath) => ({
        javaPath,
        source: "scan" as const,
      })),
    );
  }

  // Deduplicate discovered javaPath.
  const seen = new Set<string>();
  const unique = discovered.filter((d) => {
    const key = normalizePathForKey(d.javaPath);
    if (seen.has(key)) {
      return false;
    }
    seen.add(key);

    return true;
  });

  const probed: JavaCandidate[] = [];
  for (const item of unique.slice(0, maxProbeCount)) {
    try {
      // Avoid known problematic paths (PCL): system32 javapath & temp targets.
      if (isWindows()) {
        const key = normalizePathForKey(item.javaPath);
        if (
          key.includes("\\\\windows\\\\system32\\\\") ||
          key.includes("javapath_target_") ||
          key.includes("java8path_target_") ||
          key.includes("javatmp")
        ) {
          // Keep, but they should be de-prioritized by preferRoots; here we just skip probing to reduce noise.
          continue;
        }
      }

      const probe = await probeJava(item.javaPath, {
        timeoutMs: probeTimeoutMs,
      });
      if (!shouldKeepCandidate(probe, filter)) {
        continue;
      }

      // PCL special incompatible: /lib/ext exists
      if (probe.output.toLowerCase().includes("/lib/ext exists")) {
        continue;
      }
      if (probe.output.toLowerCase().includes("a fatal error")) {
        continue;
      }

      probed.push({ ...probe, source: item.source });
    } catch {
      // ignore invalid candidates
    }
  }

  // Deduplicate by homeDir.
  const homeSeen = new Set<string>();
  const deduped = probed.filter((p) => {
    const key = normalizePathForKey(p.homeDir);
    if (homeSeen.has(key)) {
      return false;
    }
    homeSeen.add(key);

    return true;
  });

  return sortJavaCandidates(deduped, { preferRoots: options?.preferRoots });
}
