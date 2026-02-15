import { useState } from "react";
import { useSettings } from "../hooks/useSettings";
import type { SettingsData } from "../lib/tauriCommands";

export function Settings() {
  const { settings, devices, loading, saveSettings } = useSettings();
  const [form, setForm] = useState<SettingsData | null>(null);
  const [saved, setSaved] = useState(false);

  // Use form state if user has started editing, otherwise use loaded settings
  const current = form ?? settings;

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <span className="text-gray-400">Loading settings...</span>
      </div>
    );
  }

  function updateField<K extends keyof SettingsData>(
    key: K,
    value: SettingsData[K]
  ) {
    setForm({ ...current, [key]: value });
    setSaved(false);
  }

  async function handleSave() {
    if (!form) return;
    await saveSettings(form);
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  }

  return (
    <div className="max-w-lg mx-auto p-6 space-y-6">
      <h1 className="text-2xl font-bold">VoiceFlow Settings</h1>

      {/* API Key */}
      <div className="space-y-2">
        <label className="block text-sm font-medium text-gray-300">
          OpenAI API Key
        </label>
        <input
          type="password"
          value={current.openai_api_key}
          onChange={(e) => updateField("openai_api_key", e.target.value)}
          placeholder="sk-..."
          className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      {/* Hotkeys */}
      <div className="grid grid-cols-2 gap-4">
        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-300">
            Dictation Hotkey
          </label>
          <input
            type="text"
            value={current.dictation_hotkey}
            onChange={(e) => updateField("dictation_hotkey", e.target.value)}
            className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-300">
            Command Hotkey
          </label>
          <input
            type="text"
            value={current.command_hotkey}
            onChange={(e) => updateField("command_hotkey", e.target.value)}
            className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
      </div>

      {/* Microphone Selection */}
      <div className="space-y-2">
        <label className="block text-sm font-medium text-gray-300">
          Microphone
        </label>
        <select
          value={current.selected_mic ?? ""}
          onChange={(e) =>
            updateField("selected_mic", e.target.value || null)
          }
          className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="">Default</option>
          {devices.map((d) => (
            <option key={d} value={d}>
              {d}
            </option>
          ))}
        </select>
      </div>

      {/* Personal Dictionary */}
      <div className="space-y-2">
        <label className="block text-sm font-medium text-gray-300">
          Personal Dictionary
        </label>
        <p className="text-xs text-gray-500">
          Comma-separated words/names to improve transcription accuracy
        </p>
        <input
          type="text"
          value={current.personal_dictionary.join(", ")}
          onChange={(e) =>
            updateField(
              "personal_dictionary",
              e.target.value
                .split(",")
                .map((s) => s.trim())
                .filter(Boolean)
            )
          }
          placeholder="VoiceFlow, Tauri, WisprFlow"
          className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      {/* Save Button */}
      <button
        onClick={handleSave}
        disabled={!form}
        className={`w-full py-2 px-4 rounded-lg font-medium transition-colors ${
          form
            ? "bg-blue-600 hover:bg-blue-700 text-white cursor-pointer"
            : "bg-gray-700 text-gray-500 cursor-not-allowed"
        }`}
      >
        {saved ? "Saved!" : "Save Settings"}
      </button>

      {/* Usage Instructions */}
      <div className="mt-8 p-4 bg-gray-800/50 rounded-lg text-sm text-gray-400 space-y-2">
        <h3 className="font-medium text-gray-300">How to use</h3>
        <p>
          <strong>Dictation:</strong> Hold{" "}
          <kbd className="px-1.5 py-0.5 bg-gray-700 rounded text-xs">
            {current.dictation_hotkey}
          </kbd>{" "}
          and speak. Release to transcribe and inject text.
        </p>
        <p>
          <strong>Command:</strong> Select text, then hold{" "}
          <kbd className="px-1.5 py-0.5 bg-gray-700 rounded text-xs">
            {current.command_hotkey}
          </kbd>{" "}
          and speak a command (e.g. "make this formal"). Release to transform.
        </p>
      </div>
    </div>
  );
}
