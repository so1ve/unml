import { loadPackagesFromCwd } from "./utils";

export const EXTENSION_NAME_RE = /^(@[\w-]+\/)?unml-extension(-[\w-]+)?$/;

export const filterExtensions = (packages: string[]) =>
  packages.filter((name) => EXTENSION_NAME_RE.test(name));

export const loadExtensionsFromCwd = (cwd = process.cwd()) =>
  loadPackagesFromCwd(cwd).then(filterExtensions);

export async function runExtensions(
  extensions?: string[],
  cwd = process.cwd(),
) {
  extensions ??= await loadExtensionsFromCwd(cwd);
  // const extensionModules = await Promise.all(
  //   extensions.map(async (name) => {
  //     const extension = await import(name);
  //     console.log(extension);

  //     return extension.default;
  //   }),
  // );
}
