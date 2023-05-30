import { loadExtensionsFromCwd } from "./utils";

export class ExtensionLoader {
  private extensions: string[] = [];
  async init() {
    this.extensions = await loadExtensionsFromCwd();
  }
}
