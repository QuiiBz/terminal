import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { WebLinksAddon } from "xterm-addon-web-links";
import { SearchAddon } from "xterm-addon-search";
import { WebglAddon } from "xterm-addon-webgl";
import { CanvasAddon } from "xterm-addon-canvas";
import "./index.css";
import "xterm/css/xterm.css";
import { theme } from "./theme";

export default function App() {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const root = document.querySelector(":root") as HTMLElement;

    for (const [key, value] of Object.entries(theme)) {
      root.style.setProperty(`--${key}`, value);
    }

    if (ref.current) {
      const terminal = new Terminal({
        fontFamily: "JetBrainsMono Nerd Font Mono",
        fontSize: 14,
        theme,
      });

      const fitAddon = new FitAddon();
      const searchAddon = new SearchAddon();

      terminal.loadAddon(fitAddon);
      terminal.loadAddon(new WebLinksAddon());
      terminal.loadAddon(searchAddon);
      // searchAddon.findNext('foo');

      terminal.open(ref.current);
      // terminal.loadAddon(new WebglAddon());
      // terminal.loadAddon(new CanvasAddon());

      function focus() {
        terminal.focus();
      }
      function resize() {
        fitAddon.fit();
      }

      focus();
      resize();

      invoke("spawn");

      terminal.onData((data) => {
        invoke("write", { data });
      });

      terminal.onResize(({ rows, cols }) => {
        invoke("resize", { rows, cols });
      });

      listen("data", (data) => {
        terminal.write(data.payload as Uint8Array);
      });

      window.addEventListener("focus", resize);
      window.addEventListener("resize", resize);

      return () => {
        window.removeEventListener("focus", resize);
        window.removeEventListener("resize", resize);

        terminal.dispose();
        invoke("dispose");
      };
    }
  }, []);

  return <div ref={ref} className="terminal" />;
}
