import { exposeNodeCommand, useUnml } from "@unml/kit";
import type { Tab } from "@unml/schema";

export async function initUi() {
  const unml = useUnml();
  const tabs: Tab[] = [];
  await unml.callHook("ui:tabs", tabs);
  exposeNodeCommand("ui:getTabs", () => tabs);
}
