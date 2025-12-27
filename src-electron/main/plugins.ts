import { executeWithUnml } from "@unml/ctx";
import builtinPlugin from "@unml-plugins/builtin-test";

import { createPluginUnmlApi } from "./core/api";

export function loadPlugins() {
  // Initialize builtin plugin
  executeWithUnml(createPluginUnmlApi("builtin-test"), () => {
    builtinPlugin.setup();
  });
}
