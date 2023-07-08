import { CONFIG_KEY, DEFAULT_CONFIG } from "./constants";
import { Theme } from "./themes";

export type Config = {
  theme: Theme;
  fontFamily: string;
  fontSize: number;
  cursorBlink: boolean;
};

export const getConfig = () => {
  const initialConfig = localStorage.getItem(CONFIG_KEY);
  let config: Config;

  if (initialConfig) {
    config = JSON.parse(initialConfig);
  } else {
    config = DEFAULT_CONFIG;
    localStorage.setItem(CONFIG_KEY, JSON.stringify(DEFAULT_CONFIG));
  }

  return config;
};

export const setConfig = (config: Config) => {
  localStorage.setItem(CONFIG_KEY, JSON.stringify(config));
};
