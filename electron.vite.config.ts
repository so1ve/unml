import { resolve } from "node:path";

import vue from "@vitejs/plugin-vue";
import { defineConfig } from "electron-vite";

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
		root: __dirname,
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
		plugins: [vue()],
	},
});
