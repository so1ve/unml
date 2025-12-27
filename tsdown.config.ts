import { defineConfig } from "tsdown";

export default defineConfig({
  workspace: {
    include: ["packages/*", "plugins/*"],
  },
  entry: ["src/index.ts"],
  dts: {
    oxc: true,
  },
  exports: true,
});
