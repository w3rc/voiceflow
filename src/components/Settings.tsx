import { useState } from "react";
import { useSettings } from "../hooks/useSettings";
import type { SettingsData } from "../lib/tauriCommands";

export function Settings() {
  const { settings, devices, loading, saveSettings } = useSettings();
  const [form, setForm] = useState<SettingsData | null>(null);
  const [saved, setSaved] = useState(false);

  const current = form ?? settings;

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <span className="text-gray-600 text-sm">Loading settings...</span>
      </div>
    );
  }

  function updateField<K extends keyof SettingsData>(key: K, value: SettingsData[K]) {
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
    <div className="h-full overflow-y-auto">
      <div className="max-w-lg mx-auto px-5 py-5 space-y-5">

        {/* API Key */}
        <div className="space-y-1.5">
          <label className="block text-xs font-medium text-gray-400 uppercase tracking-wider">
            OpenAI API Key
          </label>
          <input
            type="password"
            value={current.openai_api_key}
            onChange={(e) => updateField("openai_api_key", e.target.value)}
            placeholder="sk-..."
            className="w-full px-3 py-2 bg-gray-900 border border-gray-800 rounded-lg text-sm text-white placeholder-gray-700 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500 transition-colors"
          />
        </div>

        {/* Hotkeys */}
        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-1.5">
            <label className="block text-xs font-medium text-gray-400 uppercase tracking-wider">
              Dictation Hotkey
            </label>
            <p className="text-xs text-gray-600">Hold to record, release to stop</p>
            <input
              type="text"
              value={current.dictation_hotkey}
              onChange={(e) => updateField("dictation_hotkey", e.target.value)}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-800 rounded-lg text-sm text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500 transition-colors"
            />
          </div>
          <div className="space-y-1.5">
            <label className="block text-xs font-medium text-gray-400 uppercase tracking-wider">
              Toggle Hotkey
            </label>
            <p className="text-xs text-gray-600">Press once to start, again to stop</p>
            <input
              type="text"
              value={current.toggle_hotkey}
              onChange={(e) => updateField("toggle_hotkey", e.target.value)}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-800 rounded-lg text-sm text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500 transition-colors"
            />
          </div>
          <div className="space-y-1.5">
            <label className="block text-xs font-medium text-gray-400 uppercase tracking-wider">
              Command Hotkey
            </label>
            <p className="text-xs text-gray-600">Hold while text is selected</p>
            <input
              type="text"
              value={current.command_hotkey}
              onChange={(e) => updateField("command_hotkey", e.target.value)}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-800 rounded-lg text-sm text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500 transition-colors"
            />
          </div>
        </div>

        {/* Microphone */}
        <div className="space-y-1.5">
          <label className="block text-xs font-medium text-gray-400 uppercase tracking-wider">
            Microphone
          </label>
          <div className="relative">
            <select
              value={current.selected_mic ?? ""}
              onChange={(e) => updateField("selected_mic", e.target.value || null)}
              className="w-full appearance-none px-3 py-2 bg-gray-900 border border-gray-800 rounded-lg text-sm text-white focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500 transition-colors cursor-pointer pr-8"
            >
              <option value="">Default</option>
              {devices.map((d) => (
                <option key={d.name} value={d.name}>
                  {d.label}
                </option>
              ))}
            </select>
            <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2.5 text-gray-500">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
                <polyline points="6 9 12 15 18 9" />
              </svg>
            </div>
          </div>
        </div>

        {/* Personal Dictionary */}
        <div className="space-y-1.5">
          <label className="block text-xs font-medium text-gray-400 uppercase tracking-wider">
            Personal Dictionary
          </label>
          <p className="text-xs text-gray-600">
            Comma-separated words/names to improve transcription accuracy
          </p>
          <input
            type="text"
            value={current.personal_dictionary.join(", ")}
            onChange={(e) =>
              updateField(
                "personal_dictionary",
                e.target.value.split(",").map((s) => s.trim()).filter(Boolean)
              )
            }
            placeholder="VoiceFlow, Tauri, WisprFlow"
            className="w-full px-3 py-2 bg-gray-900 border border-gray-800 rounded-lg text-sm text-white placeholder-gray-700 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500 transition-colors"
          />
        </div>

        {/* Save Button */}
        <button
          onClick={handleSave}
          disabled={!form}
          className={`w-full py-2 px-4 rounded-lg text-sm font-medium transition-all ${
            saved
              ? "bg-green-600 text-white cursor-default"
              : form
              ? "bg-blue-600 hover:bg-blue-500 text-white cursor-pointer"
              : "bg-gray-800 text-gray-600 cursor-not-allowed"
          }`}
        >
          {saved ? "Saved!" : "Save Settings"}
        </button>

        {/* How to use */}
        <div className="p-4 bg-gray-900 rounded-lg border border-gray-800 space-y-3">
          <h3 className="text-xs font-medium text-gray-400 uppercase tracking-wider">
            How to use
          </h3>
          <div className="space-y-2 text-sm text-gray-500">
            <p>
              <span className="text-gray-300 font-medium">Dictation:</span>{" "}
              Hold{" "}
              <kbd className="px-1.5 py-0.5 bg-gray-800 border border-gray-700 rounded text-xs text-gray-300">
                {current.dictation_hotkey}
              </kbd>{" "}
              and speak. Release to transcribe and inject text.
            </p>
            <p>
              <span className="text-gray-300 font-medium">Toggle:</span>{" "}
              Press{" "}
              <kbd className="px-1.5 py-0.5 bg-gray-800 border border-gray-700 rounded text-xs text-gray-300">
                {current.toggle_hotkey}
              </kbd>{" "}
              once to start recording, press again to stop and inject.
            </p>
            <p>
              <span className="text-gray-300 font-medium">Command:</span>{" "}
              Select text, then hold{" "}
              <kbd className="px-1.5 py-0.5 bg-gray-800 border border-gray-700 rounded text-xs text-gray-300">
                {current.command_hotkey}
              </kbd>{" "}
              and speak a command (e.g. "make this formal"). Release to transform.
            </p>
          </div>
        </div>

      </div>
    </div>
  );
}
