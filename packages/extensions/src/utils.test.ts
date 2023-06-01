import { describe, expect, it } from "vitest";

import { EXTENSION_RE, PLAIN_PACKAGE_RE } from "./utils";

describe("extensions / utils", () => {
  it("EXTENSION_RE", () => {
    expect(EXTENSION_RE.test("@scope/unml-extension-name")).toBeTruthy();
    expect(EXTENSION_RE.test("@scope/unml-extension")).toBeTruthy();
    expect(EXTENSION_RE.test("@scope/unml-extension-name-1")).toBeTruthy();
    expect(EXTENSION_RE.test("unml-extension-name")).toBeTruthy();
    expect(EXTENSION_RE.test("unml-extension")).toBeFalsy();
  });

  it("PLAIN_PACKAGE_RE", () => {
    expect(PLAIN_PACKAGE_RE.test("@scope/unml-extension-name")).toBeFalsy();
    expect(PLAIN_PACKAGE_RE.test("@scope/unml-extension")).toBeFalsy();
    expect(PLAIN_PACKAGE_RE.test("@scope/unml-extension-name-1")).toBeFalsy();
    expect(PLAIN_PACKAGE_RE.test("unml-extension-name")).toBeTruthy();
    expect(PLAIN_PACKAGE_RE.test("unml-extension")).toBeTruthy();
  });
});
