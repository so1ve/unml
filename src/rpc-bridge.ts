// This script is loaded in the main renderer process (Host Window)
// It listens for messages from plugin iframes and forwards them to the main process via IPC

import {
  RPC_PUSH_CHANNEL,
  RPC_REQUEST_CHANNEL,
  RPC_RESPONSE_CHANNEL,
} from "@unml/ui-kit";

window.addEventListener("message", async (event) => {
  const data = event.data;

  // Filter for UNML RPC requests
  if (data?.type !== RPC_REQUEST_CHANNEL) {
    return;
  }

  // eslint-disable-next-line no-console
  console.log("Renderer received RPC request:", data);

  try {
    // Forward to main process
    const result = await window.electron.ipcRenderer.invoke("plugin:rpc", {
      pluginId: data.pluginId,
      command: data.command,
      args: data.args,
    });

    // Send response back to iframe
    // In production, we should check event.source and use it to postMessage back
    // For now, assuming the source is a window object
    if (event.source) {
      (event.source as Window).postMessage(
        {
          type: RPC_RESPONSE_CHANNEL,
          requestId: data.requestId,
          result,
        },
        { targetOrigin: event.origin },
      );
    }
  } catch (error: any) {
    console.error("RPC Error:", error);
    if (event.source) {
      (event.source as Window).postMessage(
        {
          type: RPC_RESPONSE_CHANNEL,
          requestId: data.requestId,
          error: error.message ?? String(error),
        },
        { targetOrigin: event.origin },
      );
    }
  }
});

// Listen for RPC push messages from main process
window.electron.ipcRenderer.on(
  RPC_PUSH_CHANNEL,
  (_event, { pluginId, event: eventName, data }) => {
    // eslint-disable-next-line no-console
    console.log(`Received RPC push for ${pluginId}: ${eventName}`, data);

    // Forward to all iframes that match the pluginId
    // Since we don't have a direct map of iframes, we can iterate or use postMessage to all
    // For security, we should only send to iframes with the correct origin
    const frames = document.querySelectorAll("iframe");
    for (const frame of frames) {
      try {
        const src = frame.src;
        if (src && new URL(src).hostname === pluginId) {
          frame.contentWindow?.postMessage(
            {
              type: RPC_PUSH_CHANNEL,
              event: eventName,
              data,
            },
            "*", // We can restrict this to the specific origin if we parse it
          );
        }
      } catch (e) {
        console.error("Error forwarding RPC push:", e);
      }
    }
  },
);
