import { createUnml, initUnml } from "@unml/core";
import { app } from "electron";
import electronDebug from "electron-debug";

import { loadExtensions } from "./extensions";
import { loadHooks } from "./hooks";
import { initProtocol, preInitProtocol } from "./protocol";
import { initUi } from "./ui";
import { createWindow } from "./window";

preInitProtocol();

app
  .whenReady()
  .then(async () => initUnml(createUnml()))
  .then(async () => initProtocol())
  .then(createWindow)
  .then((win) => loadHooks({ win }))
  .then(loadExtensions)
  .then(initUi)
  .then(electronDebug);
