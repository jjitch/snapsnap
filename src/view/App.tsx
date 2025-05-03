import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Image } from "./Image";

function App() {
  const [imageData, setImageData] = useState<Uint8Array>(new Uint8Array());
  return (
    <main className="container">
      <button
        type="button"
        onClick={() =>
          invoke("one_time_shoot").then((res) =>
            setImageData(res as Uint8Array),
          )
        }
      >
        Invoke Rust command
      </button>
      <Image png_binary={imageData} />
    </main>
  );
}

export default App;
