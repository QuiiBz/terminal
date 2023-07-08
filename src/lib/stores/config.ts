import { create } from "zustand";
import { Config, getConfig, setConfig } from "../config";

const config = getConfig();

type ConfigStore = {
  config: Config;
  initialConfig: Config;
  setConfig: (config: Config) => void;
};

export const useConfig = create<ConfigStore>((set) => ({
  config,
  initialConfig: config,
  setConfig: (config: Config) => {
    set({ config });
  },
  saveConfig: (config: Config) => {
    set({ config });
    setConfig(config);
  },
}));
