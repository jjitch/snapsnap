import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { Image } from "@tauri-apps/api/image";
import "./App.css";

function App() {
  return (
    <main className="container">
      <p>Tauri</p>
    </main>
  );
}

export default App;
