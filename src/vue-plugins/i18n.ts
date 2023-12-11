import messages from "@intlify/unplugin-vue-i18n/messages";
import { locale } from "intl-locale";
import { createI18n } from "vue-i18n";

const i18n = createI18n({
	legacy: false,
	locale,
	fallbackLocale: "en",
	messages,
});

export default i18n;
