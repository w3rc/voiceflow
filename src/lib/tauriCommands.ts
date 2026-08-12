import { invoke } from "@tauri-apps/api/core";

export interface ContextInfo {
  window_title: string;
  process_name: string;
  context_description: string;
}

export interface SettingsData {
  openai_api_key: string;
  dictation_hotkey: string;
  command_hotkey: string;
  toggle_hotkey: string;
  selected_mic: string | null;
  personal_dictionary: string[];
}

export async function startRecording(): Promise<void> {
  return invoke("start_recording");
}

export async function stopRecording(): Promise<number[]> {
  return invoke("stop_recording");
}

export interface AudioDevice {
  name: string;   // internal source name used for recording
  label: string;  // human-readable description shown in UI
}

export async function listAudioDevices(): Promise<AudioDevice[]> {
  return invoke("list_audio_devices");
}

export async function transcribeAudio(wavData: number[]): Promise<string> {
  return invoke("transcribe_audio", { wavData });
}

export async function cleanupTranscript(
  rawText: string,
  context: string
): Promise<string> {
  return invoke("cleanup_transcript", { rawText, context });
}

export async function executeVoiceCommand(
  selectedText: string,
  command: string
): Promise<string> {
  return invoke("execute_voice_command", { selectedText, command });
}

export async function injectText(text: string): Promise<void> {
  return invoke("inject_text", { text });
}

export async function getSelectedText(): Promise<string> {
  return invoke("get_selected_text");
}

export async function getContext(): Promise<ContextInfo> {
  return invoke("get_context");
}

export async function getSettings(): Promise<SettingsData> {
  return invoke("get_settings");
}

export async function updateSettings(settings: SettingsData): Promise<void> {
  return invoke("update_settings", { settings });
}
