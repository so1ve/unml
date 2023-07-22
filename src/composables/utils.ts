import type { Tab } from "@unml/schema";

export const getTabPath = (tab: Tab) =>
	"path" in tab ? tab.path : `/extension/custom/${tab.view}`;
