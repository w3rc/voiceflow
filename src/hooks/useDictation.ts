import { useCallback, useRef } from "react";
import { useAppStore } from "../stores/appStore";
import {
  startRecording,
  stopRecording,
  transcribeAudio,
  cleanupTranscript,
  injectText,
  getContext,
} from "../lib/tauriCommands";

export function useDictation() {
  const isProcessing = useRef(false);
  const setStatus = useAppStore((s) => s.setStatus);
  const setLastTranscript = useAppStore((s) => s.setLastTranscript);

  const startDictation = useCallback(async () => {
    if (isProcessing.current) return;

    try {
      setStatus("recording", "Recording...");
      await startRecording();
    } catch (err) {
      setStatus("error", `Failed to start recording: ${err}`);
    }
  }, [setStatus]);

  const stopDictation = useCallback(async () => {
    if (isProcessing.current) return;
    isProcessing.current = true;

    try {
      setStatus("processing", "Transcribing...");

      // Stop recording and get WAV data
      const wavData = await stopRecording();

      // Get active window context
      let contextDesc = "general text input";
      try {
        const ctx = await getContext();
        contextDesc = ctx.context_description;
      } catch {
        // Fallback to general context
      }

      // Transcribe audio
      const rawText = await transcribeAudio(wavData);
      if (!rawText.trim()) {
        setStatus("idle");
        return;
      }

      setStatus("processing", "Cleaning up...");

      // Clean up with GPT
      const cleanedText = await cleanupTranscript(rawText, contextDesc);
      setLastTranscript(cleanedText);

      // Inject into focused app
      setStatus("processing", "Injecting text...");
      await injectText(cleanedText);

      setStatus("done", "Text injected!");

      // Reset after brief delay
      setTimeout(() => {
        setStatus("idle");
      }, 1500);
    } catch (err) {
      setStatus("error", `${err}`);
      setTimeout(() => setStatus("idle"), 3000);
    } finally {
      isProcessing.current = false;
    }
  }, [setStatus, setLastTranscript]);

  return { startDictation, stopDictation };
}
