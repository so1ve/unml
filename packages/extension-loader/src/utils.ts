import { access, readdir } from "node:fs/promises";
import path from "node:path";

const EXTENSION_RE =
  // eslint-disable-next-line regexp/no-unused-capturing-group
  /^@(\w+)\/unml-extension(?:-([-\w]+))?$|^unml-extension-([-\w]+)$/;

const loadPackagesFromCwd = (cwd = process.cwd()) =>
  readdir(path.join(cwd, "node_modules"));

const filterExtensions = (packages: string[]) =>
  packages.filter((name) => EXTENSION_RE.test(name));

export const loadExtensionsFromCwd = (cwd = process.cwd()) =>
  loadPackagesFromCwd(cwd).then(filterExtensions);

export const exists = (d: string) =>
  access(d).then(
    () => true,
    () => false,
  );

if (import.meta.vitest) {
  const { expect, it } = import.meta.vitest;
  it("utils", () => {
    expect(EXTENSION_RE.test("@scope/unml-extension-name")).toBe(true);
    expect(EXTENSION_RE.test("@scope/unml-extension")).toBe(true);
    expect(EXTENSION_RE.test("@scope/unml-extension-name-1")).toBe(true);
    expect(EXTENSION_RE.test("unml-extension-name")).toBe(true);
    expect(EXTENSION_RE.test("unml-extension")).toBe(false);
  });
}
