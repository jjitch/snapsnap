import type { ApiClient } from "../type";

export const mockApiClient: ApiClient = {
  fetchSnapshotOccurrences: async () => {
    return {
      occurrences: [
        { unix: 1690000000 },
        { unix: 1690003600 },
        { unix: 1690007200 },
        { unix: 1690010800 },
        { unix: 1690014400 },
        { unix: 1690018000 },
      ],
    };
  },

  fetchImageSource: async (_timePoint) => {
    return {
      src: "https://placehold.jp/150x150.png",
    };
  },
};
