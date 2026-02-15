import { useCallback, useRef } from "react";
import { useAppStore } from "../stores/appStore";
import {
  getSelectedText,
  startRecording,
  stopRecording,
  transcribeAudio,
  executeVoiceCommand,
  injectText,
} from "../lib/tauriCommands";

export function useCommand() {
  const isProcessing = useRef(false);
  const selectedTextRef = useRef("");
  const setStatus = useAppStore((s) => s.setStatus);

  const startCommand = useCallback(async () => {
    if (isProcessing.current) return;

    try {
      // First, grab the selected text before recording starts
      setStatus("processing", "Grabbing selection...");
      const selected = await getSelectedText();
      selectedTextRef.current = selected;

      // Now start recording the voice command
      setStatus("recording", "Speak your command...");
      await startRecording();
    } catch (err) {
      setStatus("error", `Failed to start command mode: ${err}`);
    }
  }, [setStatus]);

  const stopCommand = useCallback(async () => {
    if (isProcessing.current) return;
    isProcessing.current = true;

    try {
      setStatus("processing", "Transcribing command...");

      const wavData = await stopRecording();
      const command = await transcribeAudio(wavData);

      if (!command.trim()) {
        setStatus("idle");
        return;
      }

      setStatus("processing", "Transforming text...");

      const transformed = await executeVoiceCommand(
        selectedTextRef.current,
        command
      );

      // Inject transformed text (replaces selection since it was selected)
      await injectText(transformed);

      setStatus("done", "Text transformed!");
      setTimeout(() => setStatus("idle"), 1500);
    } catch (err) {
      setStatus("error", `${err}`);
      setTimeout(() => setStatus("idle"), 3000);
    } finally {
      isProcessing.current = false;
    }
  }, [setStatus]);

  return { startCommand, stopCommand };
}
