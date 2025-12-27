import type { DownloadProvider, VersionManifest } from "./types";

/**
 * HTTP utility functions
 */
async function fetchJSON(url: string, options?: RequestInit): Promise<any> {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), 10_000);

  try {
    const response = await fetch(url, {
      ...options,
      signal: controller.signal,
    });
    if (!response.ok) {
      throw new Error(`Failed to fetch ${url}: ${response.statusText}`);
    }

    return response.json();
  } finally {
    clearTimeout(timeoutId);
  }
}

/**
 * Official Mojang download provider
 */
export class MojangDownloadProvider implements DownloadProvider {
  private static readonly VERSION_MANIFEST_URL =
    "https://piston-meta.mojang.com/mc/game/version_manifest.json";

  private static readonly RESOURCES_URL =
    "https://resources.download.minecraft.net";

  getVersionListURLs(): string[] {
    return [MojangDownloadProvider.VERSION_MANIFEST_URL];
  }

  getAssetObjectCandidates(assetObjectLocation: string): string[] {
    return [`${MojangDownloadProvider.RESOURCES_URL}/${assetObjectLocation}`];
  }

  async getVersionListById(id: string): Promise<VersionManifest> {
    return fetchJSON(MojangDownloadProvider.VERSION_MANIFEST_URL);
  }

  injectURL(baseURL: string): string {
    return baseURL;
  }

  getConcurrency(): number {
    return 6;
  }
}

/**
 * BMCLAPI mirror download provider
 */
export class BMCLAPIDownloadProvider implements DownloadProvider {
  private readonly apiRoot: string;
  private readonly urlReplacements: [string, string][];

  constructor(apiRoot: string = "https://bmclapi2.bangbang93.com") {
    this.apiRoot = apiRoot;
    this.urlReplacements = [
      ["https://bmclapi2.bangbang93.com", this.apiRoot],
      ["https://launchermeta.mojang.com", this.apiRoot],
      ["https://piston-meta.mojang.com", this.apiRoot],
      ["https://piston-data.mojang.com", this.apiRoot],
      ["https://launcher.mojang.com", this.apiRoot],
      ["https://libraries.minecraft.net", `${this.apiRoot}/libraries`],
      ["https://files.minecraftforge.net/maven", `${this.apiRoot}/maven`],
      ["https://maven.minecraftforge.net", `${this.apiRoot}/maven`],
      ["https://maven.neoforged.net/releases/", `${this.apiRoot}/maven/`],
      ["https://meta.fabricmc.net", `${this.apiRoot}/fabric-meta`],
      ["https://maven.fabricmc.net", `${this.apiRoot}/maven`],
    ];
  }

  getVersionListURLs(): string[] {
    return [`${this.apiRoot}/mc/game/version_manifest.json`];
  }

  getAssetObjectCandidates(assetObjectLocation: string): string[] {
    return [`${this.apiRoot}/assets/${assetObjectLocation}`];
  }

  async getVersionListById(id: string): Promise<VersionManifest> {
    const [url] = this.getVersionListURLs();

    return await fetchJSON(url);
  }

  injectURL(baseURL: string): string {
    for (const [source, target] of this.urlReplacements) {
      if (baseURL.startsWith(source)) {
        return target + baseURL.slice(source.length);
      }
    }

    return baseURL;
  }

  getConcurrency(): number {
    return Math.max(
      6,
      (typeof process !== "undefined" && process?.cpuUsage
        ? require("node:os").cpus().length
        : 1) * 2,
    );
  }
}

/**
 * Auto-selecting download provider
 */
export class AutoDownloadProvider implements DownloadProvider {
  private providers: DownloadProvider[];
  private currentProvider?: DownloadProvider;

  constructor(providers?: DownloadProvider[]) {
    this.providers = providers || [
      new BMCLAPIDownloadProvider(),
      new MojangDownloadProvider(),
    ];
  }

  async testProviderAvailability(provider: DownloadProvider): Promise<boolean> {
    try {
      const urls = provider.getVersionListURLs();
      const response = await fetch(urls[0], { method: "HEAD" });

      return response.ok;
    } catch {
      return false;
    }
  }

  async getBestProvider(): Promise<DownloadProvider> {
    if (this.currentProvider) {
      return this.currentProvider;
    }

    for (const provider of this.providers) {
      if (await this.testProviderAvailability(provider)) {
        this.currentProvider = provider;

        return provider;
      }
    }

    // Fallback to first provider
    this.currentProvider = this.providers[0];

    return this.currentProvider;
  }

  getVersionListURLs(): string[] {
    // Return URLs from the first provider as fallback
    return this.providers[0].getVersionListURLs();
  }

  getAssetObjectCandidates(assetObjectLocation: string): string[] {
    // Return candidates from the first provider as fallback
    return this.providers[0].getAssetObjectCandidates(assetObjectLocation);
  }

  async getVersionListById(id: string): Promise<VersionManifest> {
    const provider = await this.getBestProvider();

    return provider.getVersionListById(id);
  }

  async injectURL(baseURL: string): Promise<string> {
    const provider = await this.getBestProvider();

    return provider.injectURL(baseURL);
  }

  async getConcurrency(): Promise<number> {
    const provider = await this.getBestProvider();

    return provider.getConcurrency();
  }
}
