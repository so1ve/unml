import { join } from "node:path";

import type {
  DownloadProvider,
  Library,
  LibraryRule,
  MinecraftVersion,
} from "./types";
import {
  downloadFile,
  ensureDir,
  fetchJSON,
  fileExists,
  getArch,
  getPlatform,
  parallelDownload,
} from "./utils";

/**
 * Download version JSON file
 */
export async function downloadVersionJSON(
  versionId: string,
  provider: DownloadProvider,
): Promise<MinecraftVersion> {
  const manifest = await provider.getVersionListById(versionId);
  const versionInfo = manifest.versions.find((v) => v.id === versionId);

  if (!versionInfo) {
    throw new Error(`Version ${versionId} not found in manifest`);
  }

  // Download full version JSON
  const versionURL = provider.injectURL(versionInfo.url);
  const versionJSON = await fetchJSON<MinecraftVersion>(versionURL);

  return versionJSON;
}

/**
 * Download game JAR file
 */
export async function downloadGameJar(
  version: MinecraftVersion,
  destination: string,
  provider: DownloadProvider,
  checkIntegrity: boolean = true,
): Promise<void> {
  const clientDownload = version.downloads?.client;
  if (!clientDownload) {
    throw new Error(`No client download info found for version ${version.id}`);
  }

  const jarPath = join(destination, "versions", version.id, `${version.id}.jar`);

  // Check if already exists with correct size
  if (await fileExists(jarPath, clientDownload.size)) {
    return;
  }

  const url = provider.injectURL(clientDownload.url);

  await downloadFile({
    url,
    destination: jarPath,
    integrity: checkIntegrity
      ? { algorithm: "sha1", hash: clientDownload.sha1 }
      : undefined,
  });
}

/**
 * Check if library should be downloaded based on rules
 */
function shouldDownloadLibrary(library: Library): boolean {
  // No rules means always download
  if (!library.rules || library.rules.length === 0) {
    return true;
  }

  const platform = getPlatform();
  const arch = getArch();

  let allowed = false;

  for (const rule of library.rules) {
    const matchesOS = !rule.os || matchesOSRule(rule, platform, arch);

    if (rule.action === "allow") {
      if (matchesOS) {
        allowed = true;
      }
    } else if (rule.action === "disallow" && matchesOS) {
        return false;
      }
  }

  return allowed;
}

/**
 * Check if OS rule matches current platform
 */
function matchesOSRule(
  rule: LibraryRule,
  platform: string,
  arch: string,
): boolean {
  if (!rule.os) {return true;}

  if (rule.os.name) {
    const osMap: Record<string, string> = {
      windows: "windows",
      osx: "macos",
      linux: "linux",
    };
    if (osMap[rule.os.name] !== platform) {
      return false;
    }
  }

  if (rule.os.arch && rule.os.arch !== arch) {
      return false;
    }

  return true;
}

/**
 * Download library files
 */
export async function downloadLibraries(
  version: MinecraftVersion,
  destination: string,
  provider: DownloadProvider,
  checkIntegrity: boolean = true,
  concurrency?: number,
  onProgress?: (completed: number, total: number) => void,
): Promise<void> {
  if (!version.libraries || version.libraries.length === 0) {
    return;
  }

  const downloadTasks = [];
  const platform = getPlatform();

  for (const library of version.libraries) {
    // Check if library should be downloaded
    if (!shouldDownloadLibrary(library)) {
      continue;
    }

    // Handle artifact download
    if (library.downloads?.artifact) {
      const artifact = library.downloads.artifact;
      const libraryPath = join(
        destination,
        "libraries",
        ...library.name.split(":"),
      );

      downloadTasks.push({
        url: provider.injectURL(artifact.url),
        destination: libraryPath,
        integrity: checkIntegrity
          ? { algorithm: "sha1" as const, hash: artifact.sha1 }
          : undefined,
      });
    }

    // Handle natives download
    if (library.natives && library.downloads?.classifiers) {
      const nativeKey = library.natives[platform];
      if (nativeKey) {
        const native = library.downloads.classifiers[nativeKey];
        if (native) {
          const nativePath = join(
            destination,
            "libraries",
            ...library.name.split(":"),
            "natives",
          );

          downloadTasks.push({
            url: provider.injectURL(native.url),
            destination: nativePath,
            integrity: checkIntegrity
              ? { algorithm: "sha1" as const, hash: native.sha1 }
              : undefined,
          });
        }
      }
    }
  }

  // Download all libraries in parallel
  const actualConcurrency =
    concurrency || (await Promise.resolve(provider.getConcurrency()));
  await parallelDownload(downloadTasks, actualConcurrency, onProgress);
}

/**
 * Asset object type
 */
interface AssetObject {
  hash: string;
  size: number;
}

interface AssetIndex {
  objects: Record<string, AssetObject>;
}

/**
 * Download asset files
 */
export async function downloadAssets(
  version: MinecraftVersion,
  destination: string,
  provider: DownloadProvider,
  checkIntegrity: boolean = true,
  concurrency?: number,
  onProgress?: (completed: number, total: number) => void,
): Promise<void> {
  if (!version.assetIndex) {
    return;
  }

  // Download asset index JSON
  const assetIndexURL = provider.injectURL(version.assetIndex.url);
  const assetIndexPath = join(
    destination,
    "assets",
    "indexes",
    `${version.assetIndex.id}.json`,
  );

  await ensureDir(join(destination, "assets", "indexes"));

  // Download asset index if not exists
  if (!(await fileExists(assetIndexPath))) {
    await downloadFile({
      url: assetIndexURL,
      destination: assetIndexPath,
      integrity: checkIntegrity
        ? { algorithm: "sha1", hash: version.assetIndex.sha1 }
        : undefined,
    });
  }

  // Parse asset index
  const assetIndex = await fetchJSON<AssetIndex>(
    `file://${assetIndexPath}`,
  ).catch(() => {
    // Fallback: read from downloaded file
    return import("node:fs/promises").then((fs) =>
      fs.readFile(assetIndexPath, "utf-8").then((data) => JSON.parse(data)),
    );
  });

  // Create download tasks for all assets
  const downloadTasks = [];
  for (const [name, asset] of Object.entries(assetIndex.objects)) {
    const hash = asset.hash;
    const subPath = `${hash.slice(0, 2)}/${hash}`;
    const assetPath = join(destination, "assets", "objects", subPath);

    // Skip if already exists
    if (await fileExists(assetPath, asset.size)) {
      continue;
    }

    const candidates = provider.getAssetObjectCandidates(subPath);
    downloadTasks.push({
      url: candidates[0], // Use first candidate
      destination: assetPath,
      integrity: checkIntegrity
        ? { algorithm: "sha1" as const, hash }
        : undefined,
    });
  }

  // Download all assets in parallel
  const actualConcurrency =
    concurrency || (await Promise.resolve(provider.getConcurrency()));
  await parallelDownload(downloadTasks, actualConcurrency, onProgress);
}

/**
 * Download logging configuration
 */
export async function downloadLogging(
  version: MinecraftVersion,
  destination: string,
  provider: DownloadProvider,
  checkIntegrity: boolean = true,
): Promise<void> {
  const loggingConfig = version.logging?.client;
  if (!loggingConfig) {
    return;
  }

  const loggingFile = loggingConfig.file;
  const loggingPath = join(
    destination,
    "assets",
    "log_configs",
    loggingConfig.file.url.split("/").pop() || "client.xml",
  );

  await downloadFile({
    url: provider.injectURL(loggingFile.url),
    destination: loggingPath,
    integrity: checkIntegrity
      ? { algorithm: "sha1", hash: loggingFile.sha1 }
      : undefined,
  });
}
