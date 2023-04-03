import type { Request } from "@tinyhttp/app";
import type { TinyWSRequest } from "tinyws";

export type EnhancedRequest = Request & TinyWSRequest;
