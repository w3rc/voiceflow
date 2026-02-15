import { useCallback, useEffect, useState } from "react";
import { load } from "@tauri-apps/plugin-store";
import { useAppStore } from "../stores/appStore";
import {
  getSettings,
  updateSettings,
  listAudioDevices,
  type SettingsData,
} from "../lib/tauriCommands";

const STORE_KEY = "settings";

const DEFAULT_SETTINGS: SettingsData = {
  openai_api_key: "",
  dictation_hotkey: "Ctrl+Alt+D",
  command_hotkey: "Ctrl+Alt+C",
  selected_mic: null,
  personal_dictionary: [],
};

const STORE_OPTIONS = {
  defaults: { [STORE_KEY]: DEFAULT_SETTINGS },
  autoSave: true as const,
};

export function useSettings() {
  const [settings, setSettings] = useState<SettingsData>(DEFAULT_SETTINGS);
  const [devices, setDevices] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const setApiKeySet = useAppStore((s) => s.setApiKeySet);

  useEffect(() => {
    async function init() {
      try {
        // Load from persistent store
        const store = await load("settings.json", STORE_OPTIONS);
        const stored = await store.get<SettingsData>(STORE_KEY);

        if (stored) {
          setSettings(stored);
          await updateSettings(stored);
          setApiKeySet(!!stored.openai_api_key);
        } else {
          // Load from Rust state
          const current = await getSettings();
          setSettings(current);
          setApiKeySet(!!current.openai_api_key);
        }

        // Load audio devices
        const devs = await listAudioDevices();
        setDevices(devs);
      } catch (err) {
        console.error("Failed to load settings:", err);
      } finally {
        setLoading(false);
      }
    }

    init();
  }, [setApiKeySet]);

  const saveSettings = useCallback(
    async (newSettings: SettingsData) => {
      setSettings(newSettings);
      await updateSettings(newSettings);
      setApiKeySet(!!newSettings.openai_api_key);

      // Persist to store
      const store = await load("settings.json", STORE_OPTIONS);
      await store.set(STORE_KEY, newSettings);
    },
    [setApiKeySet]
  );

  return { settings, devices, loading, saveSettings };
}
