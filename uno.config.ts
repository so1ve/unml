import presetRemToPx from "@unocss/preset-rem-to-px";
import type { Theme } from "@unocss/preset-uno";
import {
	defineConfig,
	presetAttributify,
	presetIcons,
	presetUno,
	transformerVariantGroup,
} from "unocss";

// eslint-disable-next-line ts/no-unnecessary-type-arguments
export default defineConfig<Theme>({
	shortcuts: {
		"u-systembar-control": "rounded-none duration-130",
		"u-view": "h-[calc(100vh_-_32px)] m--4",
	},
	presets: [
		presetIcons({
			prefix: ["i-", ""],
			scale: 1.5,
			extraProperties: {
				"display": "inline-block",
				"vertical-align": "middle",
			},
		}),
		presetUno(),
		presetAttributify(),
		presetRemToPx(),
	],
	transformers: [transformerVariantGroup()],
	theme: {
		colors: {
			u: {
				black: "#121212",
			},
		},
	},
});
