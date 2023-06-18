import { release } from "node:os";
import { join } from "node:path";

import { createUnml, initUnml } from "@unml/core";
import type { WebPreferences } from "electron";
import { BrowserWindow, app, ipcMain, shell } from "electron";

import { loadExtensions } from "./extensions";
import { loadHooks } from "./hooks";
import { initProtocol, preInitProtocol } from "./protocol";
import type { HookRegisterContext } from "./types";
import { initUi } from "./ui";

process.env.DIST_ELECTRON = join(__dirname, "..");
process.env.DIST = join(process.env.DIST_ELECTRON, "../dist");
process.env.PUBLIC = process.env.VITE_DEV_SERVER_URL
  ? join(process.env.DIST_ELECTRON, "../public")
  : process.env.DIST;

const TITLE = "UNML";
const ICON = join(process.env.PUBLIC, "favicon.ico");
const WIDTH = 1000;
const HEIGHT = 750;
const preload = join(__dirname, "../preload/index.js");
const url = process.env.VITE_DEV_SERVER_URL;
const indexHtml = join(process.env.DIST, "index.html");
const WEB_PREFERENCES: WebPreferences = {
  preload,
  contextIsolation: false,
  nodeIntegration: true,
};

// Disable GPU Acceleration for Windows 7
if (release().startsWith("6.1")) {
  app.disableHardwareAcceleration();
}

// Set application name for Windows 10+ notifications
if (process.platform === "win32") {
  app.setAppUserModelId(app.getName());
}

if (!app.requestSingleInstanceLock()) {
  app.quit();
  process.exit(0);
}

let win: BrowserWindow | null = null;

async function createWindow() {
  win = new BrowserWindow({
    title: TITLE,
    icon: ICON,
    width: WIDTH,
    height: HEIGHT,
    webPreferences: WEB_PREFERENCES,
    resizable: false,
    titleBarStyle: "hidden",
    frame: false,
  });

  if (process.env.VITE_DEV_SERVER_URL) {
    await win.loadURL(url);
    win.webContents.openDevTools({
      mode: "detach",
    });
  } else {
    await win.loadFile(indexHtml);
  }

  win.webContents.setWindowOpenHandler(({ url }) => {
    if (url.startsWith("https:")) {
      shell.openExternal(url);
    }

    return { action: "deny" };
  });
}

app.on("window-all-closed", () => {
  win = null;
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("second-instance", () => {
  if (win) {
    if (win.isMinimized()) {
      win.restore();
    }
    win.focus();
  }
});

app.on("activate", () => {
  const allWindows = BrowserWindow.getAllWindows();
  if (allWindows.length > 0) {
    allWindows[0].focus();
  } else {
    createWindow();
  }
});

ipcMain.handle("open-win", (_, arg) => {
  const childWindow = new BrowserWindow({
    webPreferences: WEB_PREFERENCES,
  });

  if (process.env.VITE_DEV_SERVER_URL) {
    childWindow.loadURL(`${url}#${arg as string}`);
  } else {
    childWindow.loadFile(indexHtml, { hash: arg });
  }
});

preInitProtocol();

app.whenReady().then(startApp);

async function startApp() {
  initUnml(createUnml());
  initProtocol();
  await loadExtensions();
  await initUi();
  await createWindow();
  const hookRegisterContext: HookRegisterContext = { win: win! };
  await loadHooks(hookRegisterContext);
}
