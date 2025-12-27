import type { JavaArch, JavaVersion } from "../types";

export function parseJavaVersionFromOutput(output: string): JavaVersion {
  // Prefer `version "..."`
  const m1 = /version\s+"([^"]+)"/i.exec(output);
  const m2 = /openjdk\s+(\d+(?:\.\d+)*)/i.exec(output);
  const raw = (m1?.[1] ?? m2?.[1] ?? "").trim();

  // Normalize like PCL:
  // - replace '_' with '.'
  // - keep before '-' (e.g. 17.0.10+7 or 17.0.10-ea)
  let normalized = raw.replaceAll("_", ".").split("-")[0].split("+")[0].trim();

  // Some vendors produce odd strings like "21.0.2.0.2"; PCL replaces ".0." with "." once.
  if (normalized.split(".").length > 4) {
    normalized = normalized.replace(".0.", ".");
  }

  const nums = normalized
    .split(".")
    .map((s) => s.trim())
    .filter(Boolean)
    .map((s) => Number.parseInt(s, 10))
    .filter((n) => Number.isFinite(n));

  // Ensure 4 parts.
  // PCL: if starts with 1., append 0s; else prefix 1.
  const startsWith1 = normalized.startsWith("1.");
  let parts: number[];
  if (startsWith1) {
    parts = nums;
    while (parts.length < 4) {
      parts.push(0);
    }
  } else {
    // Convert X.Y.Z into 1.X.Y.Z
    parts = [1, ...nums];
    while (parts.length < 4) {
      parts.push(0);
    }
  }

  parts = parts.slice(0, 4);

  const major = parts[0] === 1 ? parts[1] : parts[0];

  // Guard against nonsense like 1.0.* or 1.999.*
  if (parts[1] <= 0 || parts[1] >= 1000) {
    // best-effort fallback
    const altMajor = Number.isFinite(nums[0]) ? nums[0] : 0;

    return {
      raw,
      parts: [1, altMajor, 0, 0],
      major: altMajor,
    };
  }

  // PCL special-case: if minor==0 then treat as 1.X.*
  if (parts[0] !== 1 && parts[1] === 0) {
    return {
      raw,
      parts: [1, parts[0], parts[2] ?? 0, parts[3] ?? 0],
      major: parts[0],
    };
  }

  return {
    raw,
    parts: [parts[0], parts[1], parts[2], parts[3]],
    major,
  };
}

export function parseArchFromOutput(output: string): {
  arch: JavaArch;
  is64Bit?: boolean;
} {
  const lower = output.toLowerCase();
  if (lower.includes("64-bit")) {
    return { arch: "x64", is64Bit: true };
  }
  if (lower.includes("32-bit")) {
    return { arch: "x86", is64Bit: false };
  }

  // From -XshowSettings:properties (stderr) style.
  // sun.arch.data.model = 64
  const m = /sun\.arch\.data\.model\s*=\s*(\d+)/i.exec(output);
  if (m?.[1] === "64") {
    return { arch: "x64", is64Bit: true };
  }
  if (m?.[1] === "32") {
    return { arch: "x86", is64Bit: false };
  }

  const m2 = /os\.arch\s*=\s*([\w-]+)/i.exec(output);
  const arch = (m2?.[1] ?? "").toLowerCase();
  if (arch === "amd64" || arch === "x86_64") {
    return { arch: "x64", is64Bit: true };
  }
  if (arch === "x86" || arch === "i386" || arch === "i686") {
    return { arch: "x86", is64Bit: false };
  }
  if (arch === "aarch64" || arch === "arm64") {
    return { arch: "arm64", is64Bit: true };
  }

  return { arch: "unknown" };
}

export function detectVendor(output: string): string | undefined {
  const lower = output.toLowerCase();
  if (lower.includes("eclipse") && lower.includes("temurin")) {
    return "Eclipse Temurin";
  }
  if (lower.includes("adoptium")) {
    return "Eclipse Adoptium";
  }
  if (lower.includes("zulu")) {
    return "Azul Zulu";
  }
  if (lower.includes("corretto")) {
    return "Amazon Corretto";
  }
  if (lower.includes("oracle")) {
    return "Oracle";
  }
  if (lower.includes("microsoft")) {
    return "Microsoft";
  }
  if (lower.includes("openjdk")) {
    return "OpenJDK";
  }

  return undefined;
}
