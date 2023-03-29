import { rmSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig } from "vite";
// Force import order
import VueI18n from "@intlify/unplugin-vue-i18n/vite";
import Vue from "@vitejs/plugin-vue";
import Unocss from "unocss/vite";
import AutoImport from "unplugin-auto-import/vite";
import VueComponents from "unplugin-vue-components/vite";
import Electron from "vite-plugin-electron";
import ElectronRenderer from "vite-plugin-electron-renderer";
import Pages from "vite-plugin-pages";
import Layouts from "vite-plugin-vue-layouts";
import Vuetify, { transformAssetUrls } from "vite-plugin-vuetify";

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
      Vue({
        template: { transformAssetUrls },
      }),
      Vuetify({
        autoImport: true,
        styles: {
          configFile: "src/styles/settings.scss",
        },
      }),
      Pages(),
      Layouts(),
      AutoImport({
        dts: "src/auto-imports.d.ts",
        imports: [
          "vue",
          "vue-i18n",
          "vue-router",
          "pinia",
          "@vueuse/core",
          {
            vuetify: ["useTheme"],
          },
        ],
        dirs: [
          "src/composables",
          "src/stores",
        ],
        vueTemplate: true,
      }),
      VueComponents({
        dts: "src/components.d.ts",
      }),
      Unocss(),
      VueI18n({
        runtimeOnly: true,
        compositionOnly: true,
        fullInstall: true,
        include: [resolve(__dirname, "src/locales/**")],
      }),
      Electron([
        {
          entry: "electron/main/index.ts",
          vite: {
            build: {
              sourcemap,
              minify: isBuild,
              outDir: "dist-electron/main",
              rollupOptions: {
                external: Object.keys("dependencies" in (pkg as any) ? (pkg as any).dependencies : {}),
              },
            },
          },
        },
      ]),
      ElectronRenderer(),
    ],
    resolve: {
      alias: {
        "@": fileURLToPath(new URL("./src", import.meta.url)),
      },
    },
    server: {
      host: HOST,
      port: PORT,
    },
    clearScreen: false,
  };
});
