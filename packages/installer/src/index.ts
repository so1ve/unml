import type { MinecraftLocation, ResolvedVersion } from "@xmcl/core";
import { Version } from "@xmcl/core";
import type { MinecraftVersion } from "@xmcl/installer";
import { getVersionList, install, installLibraries } from "@xmcl/installer";

export class Installer {
  constructor(private minecraftLocation: MinecraftLocation) {}

  async listMinecraftVersion() {
    return (await getVersionList()).versions;
  }

  async installMinecraft(version: MinecraftVersion) {
    await install(version, this.minecraftLocation);
    const resolvedVersion: ResolvedVersion = await Version.parse(
      this.minecraftLocation,
      version.id,
    );
    await installLibraries(resolvedVersion);
  }
}
