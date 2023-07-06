import type { API_VAR, IFRAME_CLIENT_VAR } from "@unml/constants";
import type { UnmlApi, UnmlClient } from "@unml/schema";

declare global {
  interface Window {
    [API_VAR]: UnmlApi;
    [IFRAME_CLIENT_VAR]: UnmlClient;
  }
}
