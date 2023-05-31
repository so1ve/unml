export function useClient() {
  if (window.self === window.top) {
    throw new Error("useClient must be used in UNML webview");
  }
  // TODO
}
