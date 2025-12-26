import { resolve } from "node:path";

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

export default defineConfig({
	main: {
		build: {
			lib: {
				entry: resolve(__dirname, "src-electron/main/index.ts"),
			},
		},
	},
	preload: {
		build: {
			lib: {
				entry: resolve(__dirname, "src-electron/preload/index.ts"),
			},
		},
	},
	renderer: {
		root: ".",
		build: {
			rollupOptions: {
				input: resolve(__dirname, "index.html"),
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
