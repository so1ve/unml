import type { Plugin } from "@unml/kit";
import { definePlugin, useUnml } from "@unml/kit";

const builtinTestPlugin: Plugin = definePlugin({
  setup() {
    const unml = useUnml();

    unml.rpc.handle("hello", (args) => {
      console.log("Received hello from frontend:", args);

      return {
        message: `Hello from backend! You sent: ${JSON.stringify(args)}`,
      };
    });

    setInterval(() => {
      unml.rpc.send("time-update", { time: new Date().toISOString() });
    }, 500);

    console.log("Built-in plugin setup complete");
  },
});

export default builtinTestPlugin;
