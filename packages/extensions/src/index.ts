import type { Extension } from "@unml/schema";

import { NODE_MODULES_DIR, exists, loadExtensionsFromCwd } from "./utils";

export class ExtensionLoader {
  #extensions: string[] = [];
  #loadedExtensions: Extension[] = [];
  #initialized = false;
  #loaded = false;

  async init() {
    this.#extensions = (await exists(NODE_MODULES_DIR))
      ? await loadExtensionsFromCwd()
      : [];
    this.#initialized = true;
  }

  async load() {
    if (!this.#initialized) {
      throw new Error("ExtensionLoader is not initialized");
    }
    for (const extension of this.#extensions) {
      this.#loadedExtensions.push(await import(extension));
    }
    this.#loaded = true;
  }

  async run() {
    if (!this.#loaded) {
      throw new Error("ExtensionLoader is not loaded");
    }
    for (const extension of this.#loadedExtensions) {
      await extension.activate();
    }
  }
}
