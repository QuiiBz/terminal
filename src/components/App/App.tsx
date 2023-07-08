import { useEffect } from "react";
import { Terminal } from "../";
import "./app.css";
import { useConfig } from "../../lib/stores/config";

export const App = () => {
  const { config, initialConfig, setConfig } = useConfig();

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      // Increase font size
      if (event.metaKey && event.code === "Equal") {
        setConfig({
          ...config,
          fontSize: config.fontSize + 1,
        });
      }

      // Decrease font size
      if (event.metaKey && event.code === "Minus") {
        setConfig({
          ...config,
          fontSize: config.fontSize - 1,
        });
      }

      // Reset font size
      if (event.metaKey && event.code === "Digit0") {
        setConfig({
          ...config,
          fontSize: initialConfig.fontSize,
        });
      }
    };

    window.addEventListener("keydown", onKeyDown);

    return () => {
      window.removeEventListener("keydown", onKeyDown);
    };
  }, [config]);

  return <Terminal />;
};
