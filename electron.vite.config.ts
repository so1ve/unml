import { join, resolve } from "node:path";

import Vue from "@vitejs/plugin-vue";
import { defineConfig } from "electron-vite";
import UnoCSS from "unocss/vite";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import VueMacros from "unplugin-vue-macros/vite";
import { VueRouterAutoImports } from "unplugin-vue-router";
import VueRouter from "unplugin-vue-router/vite";
import VueDevTools from "vite-plugin-vue-devtools";
import MetaLayouts from "vite-plugin-vue-meta-layouts";

const r = (p: string) => resolve(__dirname, p);
const DIST_ELECTRON = r("dist-electron");

export default defineConfig({
  main: {
    build: {
      outDir: join(DIST_ELECTRON, "main"),
      lib: {
        entry: r("src-electron/main/index.ts"),
      },
    },
  },
  preload: {
    build: {
      outDir: join(DIST_ELECTRON, "preload"),
      lib: {
        entry: r("src-electron/preload/index.ts"),
      },
    },
  },
  renderer: {
    root: ".",
    build: {
      outDir: join(DIST_ELECTRON, "renderer"),
      rollupOptions: {
        input: r("index.html"),
      },
    },
    resolve: {
      // alias: {
      // 	"@renderer": resolve("src/renderer/src"),
      // },
    },
    plugins: [
      VueMacros({
        plugins: {
          vue: Vue(),
          vueRouter: VueRouter({
            extensions: [".vue", ".md"],
            exclude: ["node_modules"],
            dts: "src/typed-router.d.ts",
          }),
        },
      }),
      MetaLayouts(),
      AutoImport({
        imports: ["vue", VueRouterAutoImports],
        dts: "src/auto-imports.d.ts",
      }),
      Components({
        extensions: ["vue"],
        dts: "src/components.d.ts",
        dirs: ["src/components"],
      }),
      UnoCSS(),
      VueDevTools(),
    ],
  },
});
