import { readFile } from "node:fs/promises";
import { join, resolve } from "node:path";

import { electronApp, is, optimizer } from "@electron-toolkit/utils";
import { BrowserWindow, app, ipcMain, protocol, shell } from "electron";
import { lookup } from "mrmime";

import icon from "../../resources/icon.png?asset";
import { unmlApiImpl } from "./core/api";
import { loadPlugins } from "./plugins";

// Register custom protocol for plugins
protocol.registerSchemesAsPrivileged([
  {
    scheme: "unml-plugin",
    privileges: {
      standard: true,
      secure: true,
      supportFetchAPI: true,
      corsEnabled: true,
    },
  },
]);

function createWindow(): void {
  // Create the browser window.
  const mainWindow = new BrowserWindow({
    width: 900,
    height: 670,
    show: false,
    autoHideMenuBar: true,
    ...(process.platform === "linux" ? { icon } : {}),
    webPreferences: {
      preload: join(__dirname, "../preload/index.js"),
      sandbox: false,
    },
  });

  mainWindow.on("ready-to-show", () => {
    mainWindow.show();
  });

  mainWindow.webContents.setWindowOpenHandler((details) => {
    shell.openExternal(details.url);

    return { action: "deny" };
  });

  // HMR for renderer base on electron-vite cli.
  // Load the remote URL for development or the local html file for production.
  if (is.dev && process.env.ELECTRON_RENDERER_URL) {
    mainWindow.loadURL(process.env.ELECTRON_RENDERER_URL);
  } else {
    mainWindow.loadFile(join(__dirname, "../renderer/index.html"));
  }
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.whenReady().then(() => {
  // Set app user model id for windows
  electronApp.setAppUserModelId("dev.so1ve");

  // Default open or close DevTools by F12 in development
  // and ignore CommandOrControl + R in production.
  // see https://github.com/alex8088/electron-toolkit/tree/master/packages/utils
  app.on("browser-window-created", (_, window) => {
    optimizer.watchWindowShortcuts(window);
  });

  // Handle plugin protocol
  protocol.handle("unml-plugin", async (request) => {
    const url = new URL(request.url);
    const pluginId = url.hostname;
    const filePath = url.pathname.slice(1); // remove leading slash

    // Construct path to the file
    // In dev: workspace/plugins/<id>/<static-dir>/<file>
    // We assume we are in dev and it's at ../../plugins relative to __dirname (which is dist-electron/main)
    const pluginDir = resolve(__dirname, "../../plugins");
    const pluginRoot = join(pluginDir, pluginId);
    const packageJsonPath = join(pluginRoot, "package.json");

    let staticDir = "assets";
    try {
      const pkgContent = await readFile(packageJsonPath, "utf-8");
      const pkg = JSON.parse(pkgContent);
      staticDir = pkg.unml?.contributions?.static ?? "assets";
    } catch (e) {
      console.warn(
        `Failed to read package.json for plugin ${pluginId}, falling back to 'assets'`,
        e,
      );
    }

    const targetFile = join(pluginRoot, staticDir, filePath);

    try {
      const content = await readFile(targetFile);
      const mimeType = lookup(filePath) ?? "application/octet-stream";

      return new Response(content, {
        headers: { "content-type": mimeType },
      });
    } catch (e) {
      console.error(`Failed to load plugin file: ${targetFile}`, e);

      return new Response("Not Found", { status: 404 });
    }
  });

  loadPlugins();

  // Handle RPC messages from renderer
  ipcMain.handle("plugin:rpc", async (_, { pluginId, command, args }) => {
    console.warn(`RPC call from ${pluginId}: ${command}`, args);

    // In a real implementation, we would route this to the specific plugin instance
    // For now, we just use the global API which the builtin plugin registered with
    return await unmlApiImpl.rpc.call(command, args);
  });

  // IPC test
  // eslint-disable-next-line no-console
  ipcMain.on("ping", () => console.log("pong"));

  createWindow();

  app.on("activate", () => {
    // On macOS it's common to re-create a window in the app when the
    // dock icon is clicked and there are no other windows open.
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and require them here.
