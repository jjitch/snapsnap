import { Image } from "./Image";

function App() {
  return (
    <main className="container">
      {Array.from({ length: 10 }).map((_, index) => (
        // Suppress biome warning as this is a simple example
        // biome-ignore lint/suspicious/noArrayIndexKey:
        <div key={index}>
          <Image timePoint={{ unix: index }} />
        </div>
      ))}
    </main>
  );
}

export default App;
