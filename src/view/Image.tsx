import { useEffect, useState } from "react";
import { apiClinet } from "@/api-client";
import type { TimePoint } from "@/api-client/type";

type ImageProps = {
  timePoint: TimePoint;
};

type FetchStateLoading = {
  state: "loading";
};

type FetchStateSuccess = {
  state: "success";
  src: string;
};

type FetchStateError = {
  state: "error";
  msg: string;
};

type FetchState = FetchStateLoading | FetchStateSuccess | FetchStateError;

export function Image(props: ImageProps) {
  const [fetchState, setFetchState] = useState<FetchState>({
    state: "loading",
  });
  useEffect(() => {
    apiClinet
      .fetchImageSource(props.timePoint)
      .then((res) => {
        setFetchState({
          state: "success",
          src: res.src,
        });
      })
      .catch((err) => {
        setFetchState({
          state: "error",
          msg: err.message,
        });
      });
  }, [props.timePoint]);
  return fetchState.state === "success" ? (
    <img src={fetchState.src} alt="Example" />
  ) : fetchState.state === "error" ? (
    <div>Error: {fetchState.msg}</div>
  ) : (
    <div>Loading...</div>
  );
}
