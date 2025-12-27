/**
 * @unml/download-game
 * Minecraft game downloader for UNML
 */

// Export types
export type {
  Arch,
  AssetIndex,
  DownloadInfo,
  DownloadProgress,
  DownloadProvider,
  DownloadTask,
  FileDownloadOptions,
  GameDownloadOptions,
  IntegrityCheck,
  JavaVersionInfo,
  Library,
  LibraryRule,
  LoggingInfo,
  MinecraftVersion,
  Platform,
  VersionManifest,
} from "./types";

// Export providers
export {
  AutoDownloadProvider,
  BMCLAPIDownloadProvider,
  MojangDownloadProvider,
} from "./providers";

// Export manager
export {
  GameDownloader,
  createDownloader,
  downloadMinecraft,
} from "./manager";

// Export tasks
export {
  downloadAssets,
  downloadGameJar,
  downloadLibraries,
  downloadLogging,
  downloadVersionJSON,
} from "./tasks";

// Export utilities
export {
  calculateMD5,
  calculateSHA1,
  downloadFile,
  ensureDir,
  fetchJSON,
  fileExists,
  getArch,
  getPlatform,
  parallelDownload,
  verifyIntegrity,
} from "./utils";
