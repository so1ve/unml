import { exposeNodeCommand, useUnml } from "@unml/kit";
import type { Tab, View } from "@unml/schema";

async function initTabs() {
  const unml = useUnml();
  const tabs: Tab[] = [];
  await unml.callHook("ui:tabs", tabs);
  exposeNodeCommand("ui:getTabs", () => tabs);
}

async function initViews() {
  const unml = useUnml();
  const views: View[] = [];
  await unml.callHook("ui:views", views);
  exposeNodeCommand("ui:getViews", () => views);
}

export async function initUi() {
  await initTabs();
  await initViews();
}
