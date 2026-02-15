import { create } from "zustand";

export type AppStatus = "idle" | "recording" | "processing" | "done" | "error";

interface AppState {
  status: AppStatus;
  statusMessage: string;
  lastTranscript: string;
  apiKeySet: boolean;

  setStatus: (status: AppStatus, message?: string) => void;
  setLastTranscript: (text: string) => void;
  setApiKeySet: (set: boolean) => void;
}

export const useAppStore = create<AppState>((set) => ({
  status: "idle",
  statusMessage: "",
  lastTranscript: "",
  apiKeySet: false,

  setStatus: (status, message = "") => set({ status, statusMessage: message }),
  setLastTranscript: (text) => set({ lastTranscript: text }),
  setApiKeySet: (apiKeySet) => set({ apiKeySet }),
}));
