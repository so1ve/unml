import { exists, loadExtensionsFromCwd } from "./utils";

export class ExtensionLoader {
  private extensions: string[] = [];
  async init() {
    this.extensions = (await exists("node_modules"))
      ? await loadExtensionsFromCwd()
      : [];
  }
}
