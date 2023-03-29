import { release } from "node:os";
import { join } from "node:path";

import type { WebPreferences } from "electron";
import { BrowserWindow, app, ipcMain, shell } from "electron";

import registerControllers from "./controllers";

const TITLE = "UNML";
const ICON = join(process.env.PUBLIC, "favicon.ico");
const WIDTH = 1000;
const HEIGHT = 750;
const WEB_PREFERENCES: WebPreferences = {
  nodeIntegration: true,
  nodeIntegrationInSubFrames: true,
  contextIsolation: false,
};

process.env.DIST_ELECTRON = join(__dirname, "..");
process.env.DIST = join(process.env.DIST_ELECTRON, "../dist");
process.env.PUBLIC = process.env.VITE_DEV_SERVER_URL
  ? join(process.env.DIST_ELECTRON, "../public")
  : process.env.DIST;

// Disable GPU Acceleration for Windows 7
if (release().startsWith("6.1")) { app.disableHardwareAcceleration(); }

// Set application name for Windows 10+ notifications
if (process.platform === "win32") { app.setAppUserModelId(app.getName()); }

if (!app.requestSingleInstanceLock()) {
  app.quit();
  process.exit(0);
}

let win: BrowserWindow | null = null;
const url = process.env.VITE_DEV_SERVER_URL;
const indexHtml = join(process.env.DIST, "index.html");

function createWindow () {
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

  registerControllers(win);

  if (process.env.VITE_DEV_SERVER_URL) {
    win.loadURL(url);
    win.webContents.openDevTools({
      mode: "undocked",
    });
  } else {
    win.loadFile(indexHtml);
  }

  win.webContents.on("did-finish-load", () => {
    win?.webContents.send("main-process-message", new Date().toLocaleString());
  });

  win.webContents.setWindowOpenHandler(({ url }) => {
    if (url.startsWith("https:")) { shell.openExternal(url); }
    return { action: "deny" };
  });
}

app.whenReady().then(createWindow);

app.on("window-all-closed", () => {
  win = null;
  if (process.platform !== "darwin") { app.quit(); }
});

app.on("second-instance", () => {
  if (win) {
    if (win.isMinimized()) { win.restore(); }
    win.focus();
  }
});

app.on("activate", () => {
  const allWindows = BrowserWindow.getAllWindows();
  if (allWindows.length) {
    allWindows[0].focus();
  } else {
    createWindow();
  }
});

// New window example arg: new windows url
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
