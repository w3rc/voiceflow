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
import { playStartSound, playStopSound, playDoneSound } from "../lib/sounds";

export function useDictation() {
  const isProcessing = useRef(false);
  const setStatus = useAppStore((s) => s.setStatus);
  const setLastTranscript = useAppStore((s) => s.setLastTranscript);
  const addToHistory = useAppStore((s) => s.addToHistory);

  const startDictation = useCallback(async () => {
    if (isProcessing.current) return;

    try {
      setStatus("recording", "Recording...");
      await startRecording();
      playStartSound();
    } catch (err) {
      setStatus("error", `${err}`);
      setTimeout(() => setStatus("idle"), 3000);
    }
  }, [setStatus]);

  const stopDictation = useCallback(async () => {
    if (isProcessing.current) return;
    isProcessing.current = true;

    try {
      setStatus("processing", "Transcribing...");
      playStopSound();

      // Stop recording and get WAV data
      const wavData = await stopRecording();

      // Get active window context — timeout prevents X11 hangs on Wayland
      let contextDesc = "general text input";
      try {
        const timeout = new Promise<never>((_, rej) =>
          setTimeout(() => rej(new Error("getContext timeout")), 2000)
        );
        const ctx = await Promise.race([getContext(), timeout]);
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
      addToHistory(cleanedText);

      // Inject into focused app
      setStatus("processing", "Injecting text...");
      await injectText(cleanedText);

      playDoneSound();
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
  }, [setStatus, setLastTranscript, addToHistory]);

  return { startDictation, stopDictation };
}
