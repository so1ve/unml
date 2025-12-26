import type {
  CommandRegistry,
  MenuRegistry,
  RPCManager,
  ServiceManager,
  UIManager,
  UnmlAPI,
} from "@unml/kit";

class CommandRegistryImpl implements CommandRegistry {
  private commands = new Map<string, (...args: any[]) => any>();

  public register(id: string, handler: (...args: any[]) => any): void {
    if (this.commands.has(id)) {
      console.warn(`Command "${id}" is already registered.`);
    }
    this.commands.set(id, handler);
  }

  public async execute(id: string, ...args: any[]): Promise<any> {
    const handler = this.commands.get(id);
    if (!handler) {
      throw new Error(`Command "${id}" not found.`);
    }

    return handler(...args);
  }
}

class MenuRegistryImpl implements MenuRegistry {
  // Placeholder
}

class UIManagerImpl implements UIManager {
  // Placeholder
}

class RPCManagerImpl implements RPCManager {
  private handlers = new Map<string, (args: any) => any>();

  public handle(command: string, handler: (args: any) => any): void {
    this.handlers.set(command, handler);
  }

  public async call(command: string, args: any): Promise<any> {
    const handler = this.handlers.get(command);
    if (handler) {
      return handler(args);
    }
    throw new Error(`RPC handler for "${command}" not found.`);
  }
}

class ServiceManagerImpl implements ServiceManager {
  private services = new Map<string, any>();

  public register(id: string, service: any): void {
    this.services.set(id, service);
  }

  public get<T = any>(id: string): T | undefined {
    return this.services.get(id);
  }
}

export const unmlApiImpl: UnmlAPI = {
  commands: new CommandRegistryImpl(),
  menus: new MenuRegistryImpl(),
  ui: new UIManagerImpl(),
  rpc: new RPCManagerImpl(),
  services: new ServiceManagerImpl(),
};
