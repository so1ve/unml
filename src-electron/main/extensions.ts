import { ExtensionLoader } from "@unml/extensions";

let extensionLoader: ExtensionLoader | null = null;

export async function loadExtensions() {
  extensionLoader = new ExtensionLoader();
  await extensionLoader.init();
  await extensionLoader.load();
  await extensionLoader.run();
}
