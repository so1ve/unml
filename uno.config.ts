import presetRemToPx from "@unocss/preset-rem-to-px";
import type { Theme } from "@unocss/preset-uno";
import {
  defineConfig,
  presetIcons,
  presetUno,
  transformerVariantGroup,
} from "unocss";

export default defineConfig<Theme>({
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
