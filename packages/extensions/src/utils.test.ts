import { describe, expect, it } from "vitest";

import { EXTENSION_RE } from "./utils";

describe("extensions / utils", () => {
  it("utils", () => {
    expect(EXTENSION_RE.test("@scope/unml-extension-name")).toBeTruthy();
    expect(EXTENSION_RE.test("@scope/unml-extension")).toBeTruthy();
    expect(EXTENSION_RE.test("@scope/unml-extension-name-1")).toBeTruthy();
    expect(EXTENSION_RE.test("unml-extension-name")).toBeTruthy();
    expect(EXTENSION_RE.test("unml-extension")).toBeFalsy();
  });
});
