import { useEffect, useState } from "react";
import { apiClinet } from "@/api-client";
import type { TimePoint } from "@/api-client/type";

type ImageProps = {
  timePoint: TimePoint;
};

export function Image(props: ImageProps) {
  const [imgSource, setImgSource] = useState<string>("");
  useEffect(() => {
    apiClinet.fetchImageSource(props.timePoint).then((res) => {
      setImgSource(res.src);
    });
  }, [props.timePoint]);
  return imgSource.length > 0 ? (
    <img src={imgSource} alt="Example" />
  ) : (
    <div>Loading...</div>
  );
}
