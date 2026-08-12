import { useState, useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { Overlay } from "./components/Overlay";
import { Home } from "./components/Home";
import { Settings } from "./components/Settings";
import { useDictation } from "./hooks/useDictation";
import { useCommand } from "./hooks/useCommand";
import { useAppStore } from "./stores/appStore";
import { getSettings, updateSettings } from "./lib/tauriCommands";

type Tab = "home" | "settings";

function App() {
  const isOverlay = window.location.hash === "#overlay";
  const [activeTab, setActiveTab] = useState<Tab>("home");
  const { startDictation, stopDictation } = useDictation();
  const { startCommand, stopCommand } = useCommand();
  const setStatus = useAppStore((s) => s.setStatus);
  const setApiKeySet = useAppStore((s) => s.setApiKeySet);
  const apiKeySet = useAppStore((s) => s.apiKeySet);
  const status = useAppStore((s) => s.status);
  const statusRef = useRef(status);
  useEffect(() => { statusRef.current = status; }, [status]);

  // Load API key status on startup so the indicator is correct immediately
  useEffect(() => {
    getSettings().then((s) => setApiKeySet(!!s.openai_api_key)).catch(() => {});
  }, [setApiKeySet]);

  useEffect(() => {
    const unlistenPressed = listen<string>("hotkey-pressed", (event) => {
      if (event.payload === "dictation") startDictation();
      else if (event.payload === "command") startCommand();
    });

    const unlistenReleased = listen<string>("hotkey-released", (event) => {
      if (event.payload === "dictation") stopDictation();
      else if (event.payload === "command") stopCommand();
    });

    const unlistenToggle = listen<string>("hotkey-toggle", () => {
      const s = statusRef.current;
      if (s === "recording") stopDictation();
      else if (s === "idle" || s === "done" || s === "error") startDictation();
    });

    return () => {
      unlistenPressed.then((f) => f());
      unlistenReleased.then((f) => f());
      unlistenToggle.then((f) => f());
    };
  }, [startDictation, stopDictation, startCommand, stopCommand]);

  useEffect(() => {
    const unlistenError = listen<string>("dictation-error", (event) => {
      setStatus("error", event.payload);
    });
    return () => { unlistenError.then((f) => f()); };
  }, [setStatus]);

  if (isOverlay) return <Overlay />;

  const statusColors: Record<string, string> = {
    idle: "bg-gray-600",
    recording: "bg-red-500 animate-pulse",
    processing: "bg-blue-500 animate-pulse",
    done: "bg-green-500",
    error: "bg-orange-500",
  };

  return (
    <div className="flex flex-col h-screen bg-gray-950 text-white overflow-hidden">
      {/* Tab bar */}
      <div className="shrink-0 flex items-center justify-between px-3 pt-2 pb-1 border-b border-gray-800">
        <div className="flex items-center gap-2">
          <span className={`inline-block w-2 h-2 rounded-full ${statusColors[status] ?? "bg-gray-600"}`} />
          {!apiKeySet && (
            <span className="text-xs text-yellow-500/80">No API key</span>
          )}
        </div>
        <div className="flex items-center gap-1 bg-gray-900 rounded-lg p-0.5">
          <TabButton active={activeTab === "home"} onClick={() => setActiveTab("home")}>
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
              <polyline points="9 22 9 12 15 12 15 22" />
            </svg>
            Home
          </TabButton>
          <TabButton active={activeTab === "settings"} onClick={() => setActiveTab("settings")}>
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <circle cx="12" cy="12" r="3" />
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
            </svg>
            Settings
          </TabButton>
        </div>
        <div className="w-16" /> {/* spacer to balance the status dot */}
      </div>

      {/* Tab content */}
      <div className="flex-1 overflow-hidden">
        {activeTab === "home" ? (
          <Home startDictation={startDictation} stopDictation={stopDictation} />
        ) : (
          <Settings />
        )}
      </div>
    </div>
  );
}

function TabButton({
  active,
  onClick,
  children,
}: {
  active: boolean;
  onClick: () => void;
  children: React.ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      className={`flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-medium transition-all ${
        active ? "bg-gray-700 text-white shadow-sm" : "text-gray-500 hover:text-gray-300"
      }`}
    >
      {children}
    </button>
  );
}

export default App;
