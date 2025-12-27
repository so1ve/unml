/**
 * Core types for Minecraft game downloader
 */

export interface MinecraftVersion {
  id: string;
  type: "release" | "snapshot" | "old_beta" | "old_alpha";
  url: string;
  time: string;
  releaseTime: string;
  mainClass?: string;
  minimumLauncherVersion?: number;
  minecraftArguments?: string;
  assets?: string;
  assetIndex?: AssetIndex;
  downloads?: {
    client?: DownloadInfo;
    server?: DownloadInfo;
    client_mappings?: DownloadInfo;
    server_mappings?: DownloadInfo;
  };
  logging?: {
    client?: LoggingInfo;
  };
  libraries?: Library[];
  javaVersion?: JavaVersionInfo;
}

export interface AssetIndex {
  id: string;
  sha1: string;
  size: number;
  url: string;
  totalSize: number;
}

export interface DownloadInfo {
  sha1: string;
  size: number;
  url: string;
}

export interface LoggingInfo {
  argument: string;
  type: string;
  file: DownloadInfo;
}

export interface Library {
  name: string;
  downloads?: {
    artifact?: DownloadInfo;
    classifiers?: Record<string, DownloadInfo>;
  };
  rules?: LibraryRule[];
  natives?: Record<string, string>;
}

export interface LibraryRule {
  action: "allow" | "disallow";
  os?: {
    name?: string;
    version?: string;
    arch?: string;
  };
}

export interface JavaVersionInfo {
  component: string;
  majorVersion: number;
}

export interface VersionManifest {
  latest: {
    release: string;
    snapshot: string;
  };
  versions: MinecraftVersion[];
}

export interface DownloadProvider {
  /**
   * Get version list URLs
   */
  getVersionListURLs: () => string[];

  /**
   * Get asset object candidate URLs
   */
  getAssetObjectCandidates: (assetObjectLocation: string) => string[];

  /**
   * Get version list by ID
   */
  getVersionListById: (id: string) => Promise<VersionManifest>;

  /**
   * Inject URL for download source rewriting
   */
  injectURL: (baseURL: string) => string | Promise<string>;

  /**
   * Get concurrency level for downloads
   */
  getConcurrency: () => number | Promise<number>;
}

export interface DownloadTask {
  url: string;
  destination: string;
  sha1?: string;
  size?: number;
  retryCount?: number;
  timeout?: number;
}

export interface GameDownloadOptions {
  version: string;
  destination: string;
  provider?: DownloadProvider;
  includeAssets?: boolean;
  includeLibraries?: boolean;
  checkIntegrity?: boolean;
  concurrency?: number;
  onProgress?: (progress: DownloadProgress) => void;
}

export interface DownloadProgress {
  task: string;
  completed: number;
  total: number;
  speed: number;
  eta?: number;
}

export interface IntegrityCheck {
  algorithm: "sha1" | "md5";
  hash: string;
}

export interface FileDownloadOptions {
  url: string;
  destination: string;
  integrity?: IntegrityCheck;
  caching?: boolean;
  timeout?: number;
  retryCount?: number;
}

export type Platform = "windows" | "macos" | "linux";
export type Arch = "x86" | "x64" | "arm32" | "arm64";
