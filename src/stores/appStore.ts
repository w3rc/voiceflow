import { create } from "zustand";

export type AppStatus = "idle" | "recording" | "processing" | "done" | "error";

export interface TranscriptEntry {
  id: string;
  text: string;
  timestamp: Date;
}

interface AppState {
  status: AppStatus;
  statusMessage: string;
  lastTranscript: string;
  apiKeySet: boolean;
  transcriptHistory: TranscriptEntry[];

  setStatus: (status: AppStatus, message?: string) => void;
  setLastTranscript: (text: string) => void;
  setApiKeySet: (set: boolean) => void;
  addToHistory: (text: string) => void;
  clearHistory: () => void;
}

export const useAppStore = create<AppState>((set) => ({
  status: "idle",
  statusMessage: "",
  lastTranscript: "",
  apiKeySet: false,
  transcriptHistory: [],

  setStatus: (status, message = "") => set({ status, statusMessage: message }),
  setLastTranscript: (text) => set({ lastTranscript: text }),
  setApiKeySet: (apiKeySet) => set({ apiKeySet }),
  addToHistory: (text) =>
    set((state) => ({
      transcriptHistory: [
        ...state.transcriptHistory,
        { id: crypto.randomUUID(), text, timestamp: new Date() },
      ],
    })),
  clearHistory: () => set({ transcriptHistory: [] }),
}));
