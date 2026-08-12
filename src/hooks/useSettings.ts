import { useCallback, useEffect, useState } from "react";
import { useAppStore } from "../stores/appStore";
import {
  getSettings,
  updateSettings,
  listAudioDevices,
  type SettingsData,
  type AudioDevice,
} from "../lib/tauriCommands";

const DEFAULT_SETTINGS: SettingsData = {
  openai_api_key: "",
  dictation_hotkey: "Ctrl+Alt+D",
  command_hotkey: "Ctrl+Alt+C",
  toggle_hotkey: "Ctrl+Alt+S",
  selected_mic: null,
  personal_dictionary: [],
};

export function useSettings() {
  const [settings, setSettings] = useState<SettingsData>(DEFAULT_SETTINGS);
  const [devices, setDevices] = useState<AudioDevice[]>([]);
  const [loading, setLoading] = useState(true);
  const setApiKeySet = useAppStore((s) => s.setApiKeySet);

  useEffect(() => {
    async function init() {
      try {
        // Rust loads from disk on startup; just read the current state
        const current = await getSettings();
        const merged = { ...DEFAULT_SETTINGS, ...current };
        setSettings(merged);
        setApiKeySet(!!merged.openai_api_key);
      } catch (err) {
        console.error("Failed to load settings:", err);
      }

      try {
        const devs = await listAudioDevices();
        setDevices(devs);
      } catch (err) {
        console.error("Failed to list audio devices:", err);
      } finally {
        setLoading(false);
      }
    }

    init();
  }, [setApiKeySet]);

  const saveSettings = useCallback(
    async (newSettings: SettingsData) => {
      setSettings(newSettings);
      // updateSettings now persists to disk in Rust
      await updateSettings(newSettings);
      setApiKeySet(!!newSettings.openai_api_key);
    },
    [setApiKeySet]
  );

  return { settings, devices, loading, saveSettings };
}
