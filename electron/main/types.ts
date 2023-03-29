import type { BrowserWindow } from "electron";

export type Controller = (win: BrowserWindow) => void;
