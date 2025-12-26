import { createContext } from "unctx";

export interface CommandRegistry {
  register: (id: string, handler: (...args: any[]) => any) => void;
  execute: (id: string, ...args: any[]) => Promise<any>;
}

export interface MenuRegistry {
  // Menus are primarily defined in package.json, but dynamic manipulation can be added here
}

export interface UIManager {
  // UI related operations
}

export interface RPCManager {
  handle: (command: string, handler: (args: any) => any) => void;
  call: (command: string, args: any) => Promise<any>;
}

export interface ServiceManager {
  register: (id: string, service: any) => void;
  get: <T = any>(id: string) => T | undefined;
}

export interface UnmlAPI {
  commands: CommandRegistry;
  menus: MenuRegistry;
  ui: UIManager;
  rpc: RPCManager;
  services: ServiceManager;
}

const unmlCtx = createContext<UnmlAPI>();

export const useUnml: () => UnmlAPI = unmlCtx.use;
export const provideUnml: (value: UnmlAPI, replace?: boolean) => void =
  unmlCtx.set;

export interface Plugin {
  setup: () => void | Promise<void>;
  dispose?: () => void | Promise<void>;
}

export const definePlugin = (plugin: Plugin): Plugin => plugin;
