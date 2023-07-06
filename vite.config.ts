import { rmSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

import VueI18n from "@intlify/unplugin-vue-i18n/vite";
import Vue from "@vitejs/plugin-vue";
import Unocss from "unocss/vite";
import AutoImport from "unplugin-auto-import/vite";
import VueComponents from "unplugin-vue-components/vite";
import type { AliasOptions } from "vite";
import { defineConfig } from "vite";
// eslint-disable-next-line import/default
import Electron from "vite-plugin-electron";
import ElectronRenderer from "vite-plugin-electron-renderer";
import Pages from "vite-plugin-pages";
import Layouts from "vite-plugin-vue-layouts";
import Vuetify, { transformAssetUrls } from "vite-plugin-vuetify";

import pkg from "./package.json";
import tsconfig from "./tsconfig.json";

const dirname = fileURLToPath(new URL(".", import.meta.url));
const HOST = "127.0.0.1";
const PORT = 3344;
const EXTERNAL = Object.keys(
	"dependencies" in pkg ? (pkg as any).dependencies : {},
);

const tsconfigAlias = Object.fromEntries(
	Object.entries(tsconfig.compilerOptions.paths)
		.filter(([k]) => !k.endsWith("/*"))
		.map(([k, v]) => [k, v[0]]),
);

export const alias: AliasOptions = {
	...tsconfigAlias,
	"@": fileURLToPath(new URL("./src", import.meta.url)),
};

export default defineConfig(({ command }) => {
	rmSync("dist-electron", { recursive: true, force: true });

	const isServe = command === "serve";
	const isBuild = command === "build";
	const sourcemap = isServe || !!process.env.VSCODE_DEBUG;

	const makeEntry = (
		entry: string,
		outDir: string,
		{
			sourceMap = sourcemap,
		}: {
			sourceMap?: boolean | "inline" | "hidden";
		} = {},
	) => ({
		entry,
		vite: {
			build: {
				sourcemap: sourceMap,
				minify: isBuild,
				outDir,
				rollupOptions: {
					external: EXTERNAL,
				},
			},
			resolve: { alias },
		},
	});

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
					{
						"@unml/client": ["useClient"],
					},
				],
				dirs: ["src/composables", "src/stores"],
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
				include: [resolve(dirname, "src/locales/**")],
			}),
			Electron([
				makeEntry("src-electron/main/index.ts", "dist-electron/main"),
				{
					...makeEntry(
						"src-electron/preload/index.ts",
						"dist-electron/preload",
						{
							sourceMap: sourcemap ? "inline" : undefined,
						},
					),
					onstart(options) {
						// Notify the Renderer-Process to reload the page when the Preload-Scripts build is complete,
						// instead of restarting the entire Electron App.
						options.reload();
					},
				},
			]),
			ElectronRenderer(),
		],
		build: {
			rollupOptions: {
				external: EXTERNAL,
			},
		},
		resolve: { alias },
		server: {
			host: HOST,
			port: PORT,
		},
	};
});
