import { unmlCtx } from "@unml/kit";
import type { Unml, UnmlHooks } from "@unml/schema";
import { createHooks } from "hookable";

const initUnml = async (unml: Unml) => {
  unmlCtx.set(unml);
};

export function createUnml () {
  const hooks = createHooks<UnmlHooks>();

  const unml: Unml = {
    callHook: hooks.callHook,
    hook: hooks.hook,
  };

  initUnml(unml);

  return unml;
}

// export const createClientUnml = (ipcRenderer: IpcRenderer) => {
//   const hooks = createHooks<UnmlHooks>();

//   const unml: Unml = {
//     callHook: (...args) => {
//       ipcRenderer.send(`unml:callHook-${args[0]}`, ...args);
//       hooks.callHook(...args);
//       return new Promise((resolve) => {
//         ipcRenderer.once(`unml:callHook-${args[0]}:done`, (event, ...args) => {
//           resolve(args);
//         });
//       });
//     },
//     hook: (...args) => {
//       ipcRenderer.on(`unml:callHook-${args[0]}`, (event, ...args) => {
//         hooks.callHook.apply(null, args as any).then(() => {
//           ipcRenderer.send(`unml:callHook-${args[0] as string}:done`, ...args);
//         });
//       });
//       return hooks.hook(...args);
//     },
//   };

//   initUnml(unml);

//   return unml;
// };
