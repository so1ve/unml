import { defineBuildConfig } from "unbuild";

export default defineBuildConfig({
  clean: true,
  replace: {
    "import.meta.vitest": "undefined",
  },
});
