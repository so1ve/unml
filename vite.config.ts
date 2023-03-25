import { rmSync } from "node:fs";

import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";
import electron from "vite-plugin-electron";
import renderer from "vite-plugin-electron-renderer";

import pkg from "./package.json";

const HOST = "127.0.0.1";
const PORT = 3344;

export default defineConfig(({ command }) => {
  rmSync("dist-electron", { recursive: true, force: true });

  const isServe = command === "serve";
  const isBuild = command === "build";
  const sourcemap = isServe || !!process.env.VSCODE_DEBUG;

  return {
    plugins: [
      vue(),
      electron([
        {
          entry: "electron/main/index.ts",
          async onstart(options) {
            if (process.env.VSCODE_DEBUG) {
              // eslint-disable-next-line no-console
              console.log(/* For `.vscode/.debug.script.mjs` */ "[startup] Electron App");
            } else {
              await options.startup();
            }
          },
          vite: {
            build: {
              sourcemap,
              minify: isBuild,
              outDir: "dist-electron/main",
              rollupOptions: {
                external: Object.keys("dependencies" in pkg ? pkg.dependencies : {}),
              },
            },
          },
        },
        {
          entry: "electron/preload/index.ts",
          onstart(options) {
            // Notify the Renderer-Process to reload the page when the Preload-Scripts build is complete,
            // instead of restarting the entire Electron App.
            options.reload();
          },
          vite: {
            build: {
              sourcemap: sourcemap ? "inline" : undefined, // #332
              minify: isBuild,
              outDir: "dist-electron/preload",
              rollupOptions: {
                external: Object.keys("dependencies" in pkg ? pkg.dependencies : {}),
              },
            },
          },
        },
      ]),
      // Use Node.js API in the Renderer-process
      renderer({
        nodeIntegration: true,
      }),
    ],
    server: process.env.VSCODE_DEBUG && {
      host: HOST,
      port: PORT,
    },
    clearScreen: false,
  };
});
