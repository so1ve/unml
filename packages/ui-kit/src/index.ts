export const RPC_REQUEST_CHANNEL = "unml:rpc:request";
export const RPC_RESPONSE_CHANNEL = "unml:rpc:response";
export const RPC_PUSH_CHANNEL = "unml:rpc:push";

export interface RpcRequest {
  type: typeof RPC_REQUEST_CHANNEL;
  requestId: string;
  pluginId: string;
  command: string;
  args: any;
}

export interface RpcResponse {
  type: typeof RPC_RESPONSE_CHANNEL;
  requestId: string;
  result?: any;
  error?: string;
}

export interface RpcPush {
  type: typeof RPC_PUSH_CHANNEL;
  event: string;
  data: any;
}

type CallFunction = <T = unknown>(command: string, args: any) => Promise<T>;
type OnFunction = (event: string, callback: (data: any) => void) => () => void;

export function usePlugin(): {
  call: CallFunction;
  on: OnFunction;
} {
  const call: CallFunction = (command, args) =>
    new Promise((resolve, reject) => {
      const requestId = Math.random().toString(36).slice(7);

      function handler(event: MessageEvent) {
        if (
          event.data.type === RPC_RESPONSE_CHANNEL &&
          event.data.requestId === requestId
        ) {
          window.removeEventListener("message", handler);
          if (event.data.error) {
            reject(event.data.error);
          } else {
            resolve(event.data.result);
          }
        }
      }

      window.addEventListener("message", handler);

      // In a real scenario, pluginId should be injected or retrieved from context
      // For now, we might need to pass it or infer it.
      // However, the iframe URL contains the plugin ID, so the host can infer it from the sender origin/frame.
      // But the request object needs it if the host uses it for routing.
      // Let's assume the host injects the plugin ID into the window or we parse it from location.
      const pluginId = window.location.hostname;

      window.parent.postMessage(
        {
          type: RPC_REQUEST_CHANNEL,
          requestId,
          pluginId,
          command,
          args,
        } satisfies RpcRequest,
        "*",
      );
    });

  const on: OnFunction = (event, callback) => {
    function handler(e: MessageEvent) {
      if (e.data.type === RPC_PUSH_CHANNEL && e.data.event === event) {
        callback(e.data.data);
      }
    }
    window.addEventListener("message", handler);

    return () => window.removeEventListener("message", handler);
  };

  return {
    call,
    on,
  };
}
