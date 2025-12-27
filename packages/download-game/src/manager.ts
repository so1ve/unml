
import {
  AutoDownloadProvider,
  BMCLAPIDownloadProvider,
  MojangDownloadProvider,
} from "./providers";
import {
  downloadAssets,
  downloadGameJar,
  downloadLibraries,
  downloadLogging,
  downloadVersionJSON,
} from "./tasks";
import type { DownloadProvider, GameDownloadOptions } from "./types";
import { ensureDir } from "./utils";

/**
 * Game downloader manager
 */
export class GameDownloader {
  private provider: DownloadProvider;
  private destination: string;

  constructor(destination: string, provider?: DownloadProvider) {
    this.destination = destination;
    this.provider = provider || new AutoDownloadProvider();
  }

  /**
   * Get available download providers
   */
  static getProviders() {
    return {
      mojang: () => new MojangDownloadProvider(),
      bmclapi: (apiRoot?: string) => new BMCLAPIDownloadProvider(apiRoot),
      auto: () => new AutoDownloadProvider(),
    };
  }

  /**
   * Download complete game
   */
  async downloadGame(options: GameDownloadOptions): Promise<void> {
    const {
      version: versionId,
      destination = this.destination,
      provider = this.provider,
      includeAssets = true,
      includeLibraries = true,
      checkIntegrity = true,
      concurrency,
      onProgress,
    } = options;

    // Ensure destination directory exists
    await ensureDir(destination);

    // Step 1: Download version JSON
    onProgress?.({
      task: "Fetching version manifest",
      completed: 0,
      total: 1,
      speed: 0,
    });

    const versionJSON = await downloadVersionJSON(versionId, provider);

    // Step 2: Download game JAR
    onProgress?.({
      task: "Downloading game JAR",
      completed: 0,
      total: 1,
      speed: 0,
    });

    await downloadGameJar(versionJSON, destination, provider, checkIntegrity);

    // Step 3: Download libraries
    if (includeLibraries && versionJSON.libraries) {
      const libraryCount = versionJSON.libraries.length;
      onProgress?.({
        task: "Downloading libraries",
        completed: 0,
        total: libraryCount,
        speed: 0,
      });

      await downloadLibraries(
        versionJSON,
        destination,
        provider,
        checkIntegrity,
        concurrency,
        (completed, total) => {
          onProgress?.({
            task: "Downloading libraries",
            completed,
            total,
            speed: 0,
          });
        },
      );
    }

    // Step 4: Download assets
    if (includeAssets) {
      onProgress?.({
        task: "Downloading assets",
        completed: 0,
        total: 1,
        speed: 0,
      });

      await downloadAssets(
        versionJSON,
        destination,
        provider,
        checkIntegrity,
        concurrency,
        (completed, total) => {
          onProgress?.({
            task: "Downloading assets",
            completed,
            total,
            speed: 0,
          });
        },
      );
    }

    // Step 5: Download logging config
    onProgress?.({
      task: "Downloading logging configuration",
      completed: 0,
      total: 1,
      speed: 0,
    });

    await downloadLogging(versionJSON, destination, provider, checkIntegrity);

    // Complete
    onProgress?.({
      task: "Download complete",
      completed: 1,
      total: 1,
      speed: 0,
    });
  }

  /**
   * Get version list
   */
  async getVersionList() {
    const manifest = await this.provider.getVersionListById("");

    return manifest.versions;
  }

  /**
   * Get latest version
   */
  async getLatestVersion(type: "release" | "snapshot" = "release") {
    const manifest = await this.provider.getVersionListById("");

    return manifest.latest[type];
  }

  /**
   * Get version details
   */
  async getVersionDetails(versionId: string) {
    return downloadVersionJSON(versionId, this.provider);
  }

  /**
   * Set download provider
   */
  setProvider(provider: DownloadProvider): void {
    this.provider = provider;
  }

  /**
   * Get current provider
   */
  getProvider(): DownloadProvider {
    return this.provider;
  }
}

/**
 * Create a game downloader instance
 */
export const createDownloader = (destination: string, provider?: DownloadProvider): GameDownloader => (new GameDownloader(destination, provider));

/**
 * Quick download function
 */
export async function downloadMinecraft(
  options: GameDownloadOptions,
): Promise<void> {
  const downloader = new GameDownloader(
    options.destination,
    options.provider,
  );
  await downloader.downloadGame(options);
}
