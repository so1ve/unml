import { createUnml, initUnml } from "@unml/core";
import { app } from "electron";
import electronDebug from "electron-debug";

import { loadCommands } from "./commands";
import { loadExtensions } from "./extensions";
import { initProtocol, preInitProtocol } from "./protocol";
import { initUi } from "./ui";
import { createWindow } from "./window";

preInitProtocol();

app
  .whenReady()
  .then(() => initUnml(createUnml()))
  .then(initProtocol)
  .then(createWindow)
  .then((win) => loadCommands({ win }))
  .then(loadExtensions)
  .then(initUi)
  .then(() => electronDebug());
