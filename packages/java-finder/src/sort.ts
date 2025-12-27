import type { JavaCandidate, JavaSortOptions } from "./types";
import { normalizePathForKey } from "./utils/path";

export function sortJavaCandidates(
  candidates: JavaCandidate[],
  options?: JavaSortOptions,
): JavaCandidate[] {
  const preferRoots = (options?.preferRoots ?? []).map((p) =>
    normalizePathForKey(p),
  );

  const weight = [
    0, 1, 2, 3, 4, 5, 6, 14, 30, 10, 12, 15, 13, 9, 8, 7, 11, 31, 29, 16, 17,
    28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18,
  ];

  function scoreRoot(homeDir: string): number {
    if (preferRoots.length === 0) {
      return 0;
    }
    const key = normalizePathForKey(homeDir);
    for (let i = 0; i < preferRoots.length; i++) {
      if (key.startsWith(preferRoots[i])) {
        return preferRoots.length - i;
      }
    }

    return 0;
  }

  return [...candidates].sort((a, b) => {
    // 1. prefer within preferRoots
    const ar = scoreRoot(a.homeDir);
    const br = scoreRoot(b.homeDir);
    if (ar !== br) {
      return br - ar;
    }

    // 2. prefer 64-bit
    const a64 = a.is64Bit ? 1 : 0;
    const b64 = b.is64Bit ? 1 : 0;
    if (a64 !== b64) {
      return b64 - a64;
    }

    // 3. prefer JRE over JDK (PCL)
    const aj = a.isJre ? 1 : 0;
    const bj = b.isJre ? 1 : 0;
    if (aj !== bj) {
      return bj - aj;
    }

    // 4. major version weight
    if (a.version.major !== b.version.major) {
      const aw = weight[a.version.major] ?? 0;
      const bw = weight[b.version.major] ?? 0;

      return bw - aw;
    }

    // 5. revision closer to 51 (PCL)
    const aRev = a.version.parts[3];
    const bRev = b.version.parts[3];
    const aDist = Math.abs(aRev - 51);
    const bDist = Math.abs(bRev - 51);
    if (aDist !== bDist) {
      return aDist - bDist;
    }

    return a.homeDir.localeCompare(b.homeDir);
  });
}
