import type {
  CustomTab,
  ServerFunctions,
  UnmlServerContext,
} from "@unml/schema";

export function setupCustomTabRPC({ unml, refresh }: UnmlServerContext) {
  const iframeTabs: CustomTab[] = [];
  const customTabs: CustomTab[] = []; // TODO

  async function initHooks() {
    unml.hook("ui:tabs:refresh", initCustomTabs);
    await initCustomTabs();
  }

  async function initCustomTabs() {
    customTabs.length = 0;
    await unml.callHook("ui:tabs", customTabs);
    refresh("getCustomTabs");
  }

  unml.hook("app:loaded", async () => {
    await initHooks();
  });

  return {
    getCustomTabs() {
      return [...iframeTabs, ...customTabs];
    },
    // async customTabAction(name, actionIndex) {
    //   const tab = customTabs.find(i => i.name === name);
    //   if (!tab) {
    //     return false;
    //   }
    //   const view = tab.view;
    //   if (view.type !== "launch") {
    //     return false;
    //   }
    //   const action = view.actions?.[actionIndex];
    //   if (!action) {
    //     return false;
    //   }

    //   Promise.resolve(action.handle?.())
    //     .catch((e) => {
    //       console.error(e);
    //     })
    //     .finally(() => {
    //       unml.callHook("devtools:customTabs:refresh");
    //     });
    //   unml.callHook("devtools:customTabs:refresh");
    //   return true;
    // },
  } satisfies Partial<ServerFunctions>;
}
