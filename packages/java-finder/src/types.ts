export type JavaArch = "x64" | "x86" | "arm64" | "unknown";

export type JavaSource =
  | "env:JAVA_HOME"
  | "env:PATH"
  | "where"
  | "registry"
  | "common"
  | "scan";

export interface JavaVersion {
  /**
   * Normalized into 4 numeric parts, similar to PCL's `Version`.
   *
   * Examples:
   *
   * - Java 8u312 -> [1, 8, 0, 312]
   * - Java 17.0.10 -> [17, 0, 10, 0] (normalized as [1, 17, 0, 10] below)
   */
  parts: [number, number, number, number];
  /**
   * The original version string parsed from output (best-effort).
   */
  raw: string;
  /**
   * Java major version (8/11/17/21/...).
   */
  major: number;
}

export interface JavaProbeResult {
  javaPath: string;
  javawPath?: string;
  homeDir: string;
  binDir: string;

  isJre: boolean;
  is64Bit?: boolean;
  arch: JavaArch;

  version: JavaVersion;
  vendor?: string;
  /**
   * Raw combined stdout+stderr from the probe command.
   */
  output: string;
}

export interface JavaCandidate extends JavaProbeResult {
  source: JavaSource;
}

export interface FindJavaFilter {
  minMajor?: number;
  maxMajor?: number;
  /**
   * Match PCL's behavior: disallow using high-version JRE (>=16) due to
   * compatibility issues. Off by default for library usage.
   */
  disallowHighJre?: boolean;
  /**
   * Exclude `...\\.minecraft\\runtime\\...` (official launcher bundled
   * runtime).
   */
  excludeMinecraftRuntime?: boolean;
  /**
   * If true, require 64-bit Java on 64-bit OS (like PCL).
   */
  require64BitOn64BitOS?: boolean;
}

export interface FindJavaOptions {
  /**
   * Defaults to current process env.
   */
  env?: NodeJS.ProcessEnv;
  /**
   * Prefer candidates whose homeDir starts with these roots.
   */
  preferRoots?: string[];

  includeRegistry?: boolean;
  includeEnv?: boolean;
  includeWhere?: boolean;
  includeCommonLocations?: boolean;

  /**
   * Expensive on Windows: scans directories recursively using a keyword
   * heuristic. Defaults to false.
   */
  deepScan?: boolean;
  /**
   * Roots for deepScan. If omitted, uses drive roots on Windows.
   */
  deepScanRoots?: string[];
  /**
   * Max directory recursion depth for deepScan. Default 5.
   */
  deepScanMaxDepth?: number;

  /**
   * Timeout for probing `java`. Default 15000ms (matches PCL).
   */
  probeTimeoutMs?: number;
  /**
   * Max candidates to probe. Default 50.
   */
  maxProbeCount?: number;

  filter?: FindJavaFilter;
}

export interface JavaSortOptions {
  preferRoots?: string[];
}
