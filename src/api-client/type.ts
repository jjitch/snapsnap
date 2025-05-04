export interface TimePoint {
  unix: number;
}

export interface TimeRange {
  start: TimePoint;
  end: TimePoint;
}

export interface SnapshotOccurrences {
  occurrences: Array<TimePoint>;
}

export interface ImageSource {
  src: string;
}

export interface ApiClient {
  fetchSnapshotOccurrences: (
    timeRange: TimeRange,
  ) => Promise<SnapshotOccurrences>;
  fetchImageSource: (timePoint: TimePoint) => Promise<ImageSource>;
}
