import { callHook, exposeNodeCommand } from "@unml/kit";
import type { Tab, View } from "@unml/schema";

async function initTabs() {
  const tabs: Tab[] = [];
  await callHook("ui:tabs", tabs);
  exposeNodeCommand("ui:getTabs", () => tabs);
}

async function initViews() {
  const views: View[] = [];
  await callHook("ui:views", views);
  exposeNodeCommand("ui:getViews", () => views);
}

export async function initUi() {
  await initTabs();
  await initViews();
}
