import type { API_VAR } from "@unml/constants";
import type { UnmlApi } from "@unml/schema";

declare global {
  interface Window {
    [API_VAR]: UnmlApi;
  }
}
