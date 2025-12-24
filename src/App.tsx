import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="flex h-screen">
      {/* Sidebar placeholder */}
      <aside className="sidebar w-56 p-4 border-r border-macos-border dark:border-macos-border-dark">
        <h1 className="text-xl font-semibold mb-6">CleanMac</h1>
        <nav className="space-y-2">
          <div className="px-3 py-2 rounded-lg bg-macos-accent text-white">
            Dashboard
          </div>
          <div className="px-3 py-2 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer">
            System Cache
          </div>
          <div className="px-3 py-2 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer">
            Large Files
          </div>
          <div className="px-3 py-2 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer">
            Duplicates
          </div>
          <div className="px-3 py-2 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer">
            Leftovers
          </div>
        </nav>
      </aside>

      {/* Main content placeholder */}
      <main className="flex-1 p-6 overflow-auto">
        <div className="max-w-4xl mx-auto">
          <h2 className="text-2xl font-bold mb-6">Dashboard</h2>

          <div className="card mb-6">
            <h3 className="text-lg font-semibold mb-4">Quick Test</h3>
            <form
              onSubmit={(e) => {
                e.preventDefault();
                greet();
              }}
              className="flex gap-3"
            >
              <input
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Enter your name..."
                className="flex-1 px-4 py-2 border border-macos-border dark:border-macos-border-dark rounded-lg bg-transparent focus:outline-none focus:ring-2 focus:ring-macos-accent"
              />
              <button type="submit" className="btn btn-primary">
                Greet
              </button>
            </form>
            {greetMsg && (
              <p className="mt-4 text-macos-success">{greetMsg}</p>
            )}
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div className="card">
              <h3 className="text-sm text-gray-500 dark:text-gray-400 mb-1">Total Cleanable</h3>
              <p className="text-3xl font-bold">-- GB</p>
            </div>
            <div className="card">
              <h3 className="text-sm text-gray-500 dark:text-gray-400 mb-1">Disk Space Free</h3>
              <p className="text-3xl font-bold">-- GB</p>
            </div>
          </div>

          <div className="mt-6 p-4 border border-dashed border-macos-border dark:border-macos-border-dark rounded-lg text-center text-gray-500">
            <p>Project setup complete! The full UI will be implemented in subsequent steps.</p>
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;
