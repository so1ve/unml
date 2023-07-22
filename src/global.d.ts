import type { API_VAR, IFRAME_CLIENT_VAR } from "@unml/constants";
import type { Api, Client } from "@unml/schema";

declare global {
	interface Window {
		[API_VAR]: Api;
		[IFRAME_CLIENT_VAR]: Client;
	}
}
