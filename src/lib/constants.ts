import { Config } from "./config";
import { THEME_CATPPUCCIN_MACCIATO } from "./themes";

export const CONFIG_KEY = "config";

export const DEFAULT_CONFIG: Config = {
  theme: THEME_CATPPUCCIN_MACCIATO,
  fontFamily:
    "JetbrainsMono Nerd Font Mono, ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace",
  fontSize: 14,
  cursorBlink: true,
};
