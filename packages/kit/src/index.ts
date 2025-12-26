export type {
  CommandRegistry,
  MenuRegistry,
  RPCManager,
  ServiceManager,
  UIManager,
  UnmlAPI,
} from "@unml/ctx";
export { useUnml } from "@unml/ctx";

export interface Plugin {
  setup: () => void | Promise<void>;
  dispose?: () => void | Promise<void>;
}

export const definePlugin = (plugin: Plugin): Plugin => plugin;
