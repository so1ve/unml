import { createHash } from "node:crypto";
import { createWriteStream } from "node:fs";
import { mkdir, readFile, stat, unlink } from "node:fs/promises";
import { dirname } from "node:path";
import { pipeline } from "node:stream/promises";

import type { FileDownloadOptions, IntegrityCheck } from "./types";

/**
 * Calculate SHA1 hash of a file
 */
export async function calculateSHA1(filePath: string): Promise<string> {
  const fileBuffer = await readFile(filePath);
  const hash = createHash("sha1");
  hash.update(fileBuffer);

  return hash.digest("hex");
}

/**
 * Calculate MD5 hash of a file
 */
export async function calculateMD5(filePath: string): Promise<string> {
  const fileBuffer = await readFile(filePath);
  const hash = createHash("md5");
  hash.update(fileBuffer);

  return hash.digest("hex");
}

/**
 * Verify file integrity
 */
export async function verifyIntegrity(
  filePath: string,
  integrity: IntegrityCheck,
): Promise<boolean> {
  try {
    const actualHash =
      integrity.algorithm === "sha1"
        ? await calculateSHA1(filePath)
        : await calculateMD5(filePath);

    return actualHash.toLowerCase() === integrity.hash.toLowerCase();
  } catch {
    return false;
  }
}

/**
 * Check if file exists and has expected size
 */
export async function fileExists(
  filePath: string,
  expectedSize?: number,
): Promise<boolean> {
  try {
    const stats = await stat(filePath);
    if (!stats.isFile()) {return false;}
    if (expectedSize !== undefined && stats.size !== expectedSize) {return false;}

    return true;
  } catch {
    return false;
  }
}

/**
 * Ensure directory exists
 */
export async function ensureDir(dirPath: string): Promise<void> {
  await mkdir(dirPath, { recursive: true });
}

/**
 * Download file with retry and integrity check
 */
export async function downloadFile(
  options: FileDownloadOptions,
): Promise<void> {
  const {
    url,
    destination,
    integrity,
    timeout = 30_000,
    retryCount = 3,
  } = options;

  // Ensure destination directory exists
  await ensureDir(dirname(destination));

  let lastError: Error | undefined;

  for (let attempt = 0; attempt <= retryCount; attempt++) {
    try {
      // Download file
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), timeout);

      try {
        const response = await fetch(url, { signal: controller.signal });

        if (!response.ok) {
          throw new Error(
            `HTTP ${response.status}: ${response.statusText} for ${url}`,
          );
        }

        if (!response.body) {
          throw new Error(`No response body for ${url}`);
        }

        // Write to file
        const fileStream = createWriteStream(destination);
        await pipeline(response.body as any, fileStream);
      } finally {
        clearTimeout(timeoutId);
      }

      // Verify integrity if required
      if (integrity) {
        const isValid = await verifyIntegrity(destination, integrity);
        if (!isValid) {
          await unlink(destination).catch(() => {});
          throw new Error(
            `Integrity check failed for ${destination}. Expected ${integrity.algorithm}: ${integrity.hash}`,
          );
        }
      }

      // Success
      return;
    } catch (error) {
      lastError = error instanceof Error ? error : new Error(String(error));

      // Clean up partial download
      await unlink(destination).catch(() => {});

      // Don't retry on certain errors
      if (
        error instanceof Error &&
        (error.message.includes("404") || error.message.includes("403"))
      ) {
        throw error;
      }

      // Retry with exponential backoff
      if (attempt < retryCount) {
        const delay = Math.min(1000 * 2 ** attempt, 10_000);
        await new Promise((resolve) => setTimeout(resolve, delay));
      }
    }
  }

  throw new Error(
    `Failed to download ${url} after ${retryCount + 1} attempts: ${lastError?.message}`,
  );
}

/**
 * Parallel download with concurrency limit
 */
export async function parallelDownload(
  tasks: FileDownloadOptions[],
  concurrency: number,
  onProgress?: (completed: number, total: number) => void,
): Promise<void> {
  const total = tasks.length;
  let completed = 0;
  const errors: Error[] = [];

  // Create worker pool
  const workers = Array.from({ length: concurrency }, async () => {
    while (tasks.length > 0) {
      const task = tasks.shift();
      if (!task) {break;}

      try {
        await downloadFile(task);
        completed++;
        onProgress?.(completed, total);
      } catch (error) {
        errors.push(
          error instanceof Error ? error : new Error(String(error)),
        );
      }
    }
  });

  // Wait for all workers to complete
  await Promise.all(workers);

  // Throw if there were any errors
  if (errors.length > 0) {
    throw new Error(
      `Failed to download ${errors.length} files:\n${errors.map((e) => e.message).join("\n")}`,
    );
  }
}

/**
 * Fetch JSON from URL
 */
export async function fetchJSON<TResult = any>(
  url: string,
  timeout: number = 10_000,
): Promise<TResult> {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeout);

  try {
    const response = await fetch(url, { signal: controller.signal });
    if (!response.ok) {
      throw new Error(`Failed to fetch ${url}: ${response.statusText}`);
    }

    return (await response.json()) as TResult;
  } finally {
    clearTimeout(timeoutId);
  }
}

/**
 * Get current platform
 */
export function getPlatform(): "windows" | "macos" | "linux" {
  const platform = process.platform;
  switch (platform) {
    case "win32": {
      return "windows";
    }
    case "darwin": {
      return "macos";
    }
    case "linux": {
      return "linux";
    }
    default: {
      return "linux";
    }
  }
}

/**
 * Get current architecture
 */
export function getArch(): "x86" | "x64" | "arm32" | "arm64" {
  const arch = process.arch;
  switch (arch) {
    case "ia32": {
      return "x86";
    }
    case "x64": {
      return "x64";
    }
    case "arm": {
      return "arm32";
    }
    case "arm64": {
      return "arm64";
    }
    default: {
      return "x64";
    }
  }
}
