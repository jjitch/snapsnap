type ImageProps = {
  png_binary: Uint8Array;
};

export function Image(props: ImageProps) {
  if (!props.png_binary.length) {
    return (
      <div style={{ width: "100px", height: "100px", backgroundColor: "gray" }}>
        No Image
      </div>
    );
  }
  const blob = new Blob([props.png_binary], { type: "image/png" });
  const url = URL.createObjectURL(blob);
  return (
    <img
      // src={`data:image/png;base64,${img_data}`}
      src={url}
      alt="Example"
      style={{ width: "100px", height: "100px" }}
    />
  );
}
