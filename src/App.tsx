import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { Overlay } from "./components/Overlay";
import { Settings } from "./components/Settings";
import { StatusBar } from "./components/StatusBar";
import { useDictation } from "./hooks/useDictation";
import { useCommand } from "./hooks/useCommand";
import { useAppStore } from "./stores/appStore";

function App() {
  const isOverlay = window.location.hash === "#overlay";
  const { startDictation, stopDictation } = useDictation();
  const { startCommand, stopCommand } = useCommand();
  const setStatus = useAppStore((s) => s.setStatus);

  useEffect(() => {
    const unlistenPressed = listen<string>("hotkey-pressed", (event) => {
      if (event.payload === "dictation") {
        startDictation();
      } else if (event.payload === "command") {
        startCommand();
      }
    });

    const unlistenReleased = listen<string>("hotkey-released", (event) => {
      if (event.payload === "dictation") {
        stopDictation();
      } else if (event.payload === "command") {
        stopCommand();
      }
    });

    return () => {
      unlistenPressed.then((f) => f());
      unlistenReleased.then((f) => f());
    };
  }, [startDictation, stopDictation, startCommand, stopCommand]);

  useEffect(() => {
    const unlistenError = listen<string>("dictation-error", (event) => {
      setStatus("error", event.payload);
    });

    return () => {
      unlistenError.then((f) => f());
    };
  }, [setStatus]);

  if (isOverlay) {
    return <Overlay />;
  }

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      <StatusBar />
      <Settings />
    </div>
  );
}

export default App;
