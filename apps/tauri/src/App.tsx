import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from `@tauri-apps/api/core`;
import "./App.css";

import { ping } from `@tauri-apps/plugin-codepush`;



function App() {
  const abcd = 'asdf';
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [response, setResponse] = useState("");

  function updateResponse(returnValue: any) {
    setResponse((prev) => {
      return prev + `${new Date().toLocaleTimeString()} ` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + `</br>`;
    });
  }

  function _ping() {
    ping("Pong!").then(updateResponse).catch(updateResponse)
  }

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  useEffect(() => {
    _ping()
  }, [])

  return (
    <main className="container">
      <h1>Welcome to Tauri + React 안녕하세요.</h1>

      <button onClick={() => { _ping() }}> asdf </button>
      <div dangerouslySetInnerHTML={{ __html: response }} />

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
