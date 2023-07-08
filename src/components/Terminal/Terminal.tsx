import { useEffect, useRef } from "react";
import { emit, listen } from "@tauri-apps/api/event";
import { Terminal as XtermTerminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { WebLinksAddon } from "xterm-addon-web-links";
import { SearchAddon } from "xterm-addon-search";
import { WebglAddon } from "xterm-addon-webgl";
import { Unicode11Addon } from "xterm-addon-unicode11";
import "xterm/css/xterm.css";
import "./terminal.css";
import { useConfig } from "../../lib/stores/config";

export const Terminal = () => {
  const ref = useRef<HTMLDivElement>(null);
  const terminalRef = useRef<XtermTerminal>();
  const config = useConfig((state) => state.config);

  useEffect(() => {
    if (terminalRef.current) {
      terminalRef.current.options.fontSize = config.fontSize;
    }
  }, [config]);

  useEffect(() => {
    const root = document.querySelector(":root") as HTMLElement;

    for (const [key, value] of Object.entries(config.theme)) {
      root.style.setProperty(`--${key}`, value);
    }

    if (ref.current) {
      const terminal = new XtermTerminal({
        fontFamily: config.fontFamily,
        fontSize: config.fontSize,
        theme: config.theme,
        cursorBlink: config.cursorBlink,
        allowProposedApi: true,
      });

      terminalRef.current = terminal;

      const fitAddon = new FitAddon();
      const searchAddon = new SearchAddon();
      // searchAddon.findNext('foo');

      terminal.loadAddon(fitAddon);
      terminal.loadAddon(
        new WebLinksAddon((_, uri) => {
          emit("open", uri);
        })
      );
      terminal.loadAddon(searchAddon);
      terminal.loadAddon(new Unicode11Addon());

      terminal.unicode.activeVersion = "11";

      terminal.open(ref.current);
      terminal.loadAddon(new WebglAddon());

      const focus = () => terminal.focus();
      const resize = () => fitAddon.fit();

      focus();
      resize();

      emit("spawn", {
        rows: terminal.rows,
        cols: terminal.cols,
        shell: "zsh",
      });

      const textEncoder = new TextEncoder();

      terminal.onData((data) => {
        emit("write", { data: Array.from(textEncoder.encode(data)) });
      });

      terminal.onResize(({ rows, cols }) => {
        emit("resize", { rows, cols });
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
        emit("dispose");
      };
    }
  }, []);

  return <div ref={ref} className="terminal" />;
};
