import type { Extension, LoadEvent, RunEvent } from "@unml/schema";
import { exists } from "@unml/utils";

import { NODE_MODULES_DIR, loadExtensionsFromCwd } from "./utils";

export class ExtensionLoader {
  #extensions: string[] = [];
  #loadedExtensions: Extension[] = [];
  #loadEvents: LoadEvent[] = [];
  #runEvents: RunEvent[] = [];

  async init() {
    this.#extensions = (await exists(NODE_MODULES_DIR))
      ? await loadExtensionsFromCwd()
      : [];
  }

  async load() {
    for (const extension of this.#extensions) {
      this.#loadedExtensions.push((await import(extension)).default);
    }

    for (const extension of this.#loadedExtensions) {
      const { load, run } = extension ?? {};
      load && this.#loadEvents.push(load);
      run && this.#runEvents.push(run);
    }
  }

  async runLoadEvent() {
    for (const loadEvent of this.#loadEvents) {
      await loadEvent();
    }
  }

  async runRunEvent() {
    for (const runEvent of this.#runEvents) {
      await runEvent();
    }
  }
}
