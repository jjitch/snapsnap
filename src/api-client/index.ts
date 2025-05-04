import type { ApiClient } from "./type";
import { isTauri } from "@tauri-apps/api/core";
import { mockApiClient } from "./impl/mock-client";
import { tauriApiClient } from "./impl/tauri-client";

export const apiClinet: ApiClient = isTauri() ? tauriApiClient : mockApiClient;
