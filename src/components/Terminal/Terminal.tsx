import { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { Terminal as XtermTerminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { WebLinksAddon } from "xterm-addon-web-links";
import { SearchAddon } from "xterm-addon-search";
// import { WebglAddon } from "xterm-addon-webgl";
// import { CanvasAddon } from "xterm-addon-canvas";
import { THEME_CATPPUCCIN_MACCIATO } from "../../lib/themes";
import "xterm/css/xterm.css";
import "./terminal.css";
import { getConfig } from "../../lib/config";
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
      });

      terminalRef.current = terminal;

      const fitAddon = new FitAddon();
      const searchAddon = new SearchAddon();

      terminal.loadAddon(fitAddon);
      terminal.loadAddon(
        new WebLinksAddon((_, uri) => {
          invoke("open", { uri });
        })
      );
      terminal.loadAddon(searchAddon);
      // searchAddon.findNext('foo');

      terminal.open(ref.current);
      // terminal.loadAddon(new WebglAddon());
      // terminal.loadAddon(new CanvasAddon());

      const focus = () => terminal.focus();
      const resize = () => fitAddon.fit();

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
};
