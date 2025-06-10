import { path } from "@tauri-apps/api";
import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";

interface ConfigStore {
  // This file is used to store configuration settings for the application.

  // If the config modal is open
  openSettings: () => void;
  closeSettings: () => void;
  saveSettings: () => Promise<void>;
  loadSettings: () => Promise<void>;
  isModalOpen: boolean;

  // Shows images in a masonry layout if true, otherwise uses a grid layout.
  useMasonry: boolean;
  // Shows the button for opening RE files
  useREButton: boolean;
}

class ConfigStoreClass implements ConfigStore {
  private _configFilePath = "config.json";

  isModalOpen = $state(false);
  useMasonry = $state(true);
  useREButton = $state(false);

  constructor() {
    // Load from tauri config file if available
    this.loadSettings();
  }

  async loadSettings() {
    let configDir = await path.appConfigDir();
    let configFilePath = await path.join(configDir, this._configFilePath);
    try {
      const configData = await readTextFile(configFilePath);
      const config = JSON.parse(configData);
      config && Object.assign(this, config);
    } catch (error) {
      console.error("Failed to load config:", configFilePath, "error: ", error);
      // If the config file does not exist, we will create it with default values
      this.saveSettings();
    }
  }

  async saveSettings() {
    let configDir = await path.appConfigDir();
    let configFilePath = await path.join(configDir, this._configFilePath);
    try {
      const data = JSON.stringify(
        {
          useMasonry: this.useMasonry,
          useREButton: this.useREButton,
        },
        null,
        2
      );
      console.log("Saving config", data, "to:", configFilePath);
      const configData = await writeTextFile(configFilePath, data);
    } catch (error) {
      console.error("Failed to save config:", configFilePath, "error: ", error);
    }
  }

  openSettings = () => {
    this.isModalOpen = true;
  };
  closeSettings = () => {
    this.isModalOpen = false;
  };
}

export const configStore = new ConfigStoreClass();
