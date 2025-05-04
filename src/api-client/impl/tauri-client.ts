import type { ApiClient } from "../type";
import { invoke } from "@tauri-apps/api/core";

// These implementations are in progress
export const tauriApiClient: ApiClient = {
  fetchSnapshotOccurrences: async (timeRange) => {
    return invoke("fetch_snapshot_occurrences", {
      start: timeRange.start.unix,
      end: timeRange.end.unix,
    });
  },

  fetchImageSource: async (timePoint) => {
    return invoke("fetch_image_source", {
      unix: timePoint.unix,
    });
  },
};
