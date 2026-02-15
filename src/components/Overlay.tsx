import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useAppStore, type AppStatus } from "../stores/appStore";

export function Overlay() {
  const status = useAppStore((s) => s.status);
  const statusMessage = useAppStore((s) => s.statusMessage);
  const setStatus = useAppStore((s) => s.setStatus);

  // Listen for status events from main window
  useEffect(() => {
    const unlisten = listen<{ status: AppStatus; message: string }>(
      "overlay-status",
      (event) => {
        setStatus(event.payload.status, event.payload.message);
      }
    );
    return () => {
      unlisten.then((f) => f());
    };
  }, [setStatus]);

  // Show/hide overlay based on status
  useEffect(() => {
    const win = getCurrentWindow();
    if (status === "idle") {
      win.hide();
    } else {
      win.show();
    }
  }, [status]);

  return (
    <div className="flex items-center justify-center h-screen bg-transparent">
      <div
        className={`
        flex items-center gap-3 px-5 py-3 rounded-2xl shadow-2xl backdrop-blur-md
        ${status === "recording" ? "bg-red-500/90" : ""}
        ${status === "processing" ? "bg-blue-500/90" : ""}
        ${status === "done" ? "bg-green-500/90" : ""}
        ${status === "error" ? "bg-orange-500/90" : ""}
        ${status === "idle" ? "bg-gray-800/90" : ""}
      `}
      >
        <StatusIcon status={status} />
        <span className="text-white text-sm font-medium">
          {statusMessage || statusLabel(status)}
        </span>
      </div>
    </div>
  );
}

function StatusIcon({ status }: { status: AppStatus }) {
  if (status === "recording") {
    return (
      <span className="relative flex h-4 w-4">
        <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-white opacity-75" />
        <span className="relative inline-flex rounded-full h-4 w-4 bg-white" />
      </span>
    );
  }
  if (status === "processing") {
    return (
      <svg
        className="animate-spin h-4 w-4 text-white"
        viewBox="0 0 24 24"
        fill="none"
      >
        <circle
          className="opacity-25"
          cx="12"
          cy="12"
          r="10"
          stroke="currentColor"
          strokeWidth="4"
        />
        <path
          className="opacity-75"
          fill="currentColor"
          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
        />
      </svg>
    );
  }
  if (status === "done") {
    return <span className="text-white text-lg">&#10003;</span>;
  }
  if (status === "error") {
    return <span className="text-white text-lg">&#10007;</span>;
  }
  return null;
}

function statusLabel(status: AppStatus): string {
  switch (status) {
    case "recording":
      return "Recording...";
    case "processing":
      return "Processing...";
    case "done":
      return "Done!";
    case "error":
      return "Error";
    default:
      return "";
  }
}
