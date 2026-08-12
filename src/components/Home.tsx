import { useState } from "react";
import { useAppStore } from "../stores/appStore";

interface HomeProps {
  startDictation: () => void;
  stopDictation: () => void;
}

export function Home({ startDictation, stopDictation }: HomeProps) {
  const transcriptHistory = useAppStore((s) => s.transcriptHistory);
  const clearHistory = useAppStore((s) => s.clearHistory);
  const status = useAppStore((s) => s.status);
  const [copiedId, setCopiedId] = useState<string | null>(null);

  const isRecording = status === "recording";
  const isProcessing = status === "processing";
  const isError = status === "error";
  const statusMessage = useAppStore((s) => s.statusMessage);

  async function handleCopy(id: string, text: string) {
    await navigator.clipboard.writeText(text);
    setCopiedId(id);
    setTimeout(() => setCopiedId(null), 1500);
  }

  return (
    <div className="flex flex-col h-full">
      {/* History header */}
      {transcriptHistory.length > 0 && (
        <div className="flex items-center justify-between px-4 pt-3 pb-1">
          <span className="text-xs text-gray-600 uppercase tracking-wider font-medium">
            History
          </span>
          <button
            onClick={clearHistory}
            className="text-xs text-gray-600 hover:text-gray-400 transition-colors"
          >
            Clear
          </button>
        </div>
      )}

      {/* History list */}
      <div className="flex-1 overflow-y-auto px-4 py-2 space-y-2">
        {transcriptHistory.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full gap-3 select-none">
            <svg
              width="40"
              height="40"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="1.5"
              className="text-gray-700"
            >
              <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z" />
              <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
              <line x1="12" y1="19" x2="12" y2="23" />
              <line x1="8" y1="23" x2="16" y2="23" />
            </svg>
            <span className="text-sm text-gray-600">No transcripts yet</span>
            <span className="text-xs text-gray-700">Click the mic button to start recording</span>
          </div>
        ) : (
          [...transcriptHistory].reverse().map((entry) => (
            <div
              key={entry.id}
              className="group flex items-start gap-2 p-3 bg-gray-900 rounded-lg border border-gray-800 hover:border-gray-700 transition-colors"
            >
              <div className="flex-1 min-w-0">
                <p className="text-sm text-gray-100 leading-relaxed break-words">
                  {entry.text}
                </p>
                <p className="text-xs text-gray-600 mt-1">
                  {entry.timestamp.toLocaleTimeString([], {
                    hour: "2-digit",
                    minute: "2-digit",
                  })}
                </p>
              </div>
              <button
                onClick={() => handleCopy(entry.id, entry.text)}
                className="shrink-0 mt-0.5 opacity-0 group-hover:opacity-100 p-1.5 text-gray-500 hover:text-white hover:bg-gray-700 rounded-md transition-all"
                title="Copy to clipboard"
              >
                {copiedId === entry.id ? (
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
                    <polyline points="20 6 9 17 4 12" />
                  </svg>
                ) : (
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
                  </svg>
                )}
              </button>
            </div>
          ))
        )}
      </div>

      {/* Mic button */}
      <div className="shrink-0 flex flex-col items-center gap-2 px-4 py-5 border-t border-gray-800">
        {isRecording && (
          <span className="text-xs text-red-400 animate-pulse">Recording — click to stop</span>
        )}
        {isProcessing && (
          <span className="text-xs text-blue-400 animate-pulse">Processing...</span>
        )}
        {isError && (
          <span className="text-xs text-orange-400 text-center max-w-[240px]">{statusMessage || "Error"}</span>
        )}
        <button
          onClick={() => {
            if (isRecording) stopDictation();
            else if (!isProcessing) startDictation();
          }}
          disabled={isProcessing}
          className={`w-14 h-14 rounded-full flex items-center justify-center transition-all duration-150 select-none ${
            isRecording
              ? "bg-red-500 scale-110 shadow-lg shadow-red-500/25 ring-4 ring-red-500/20"
              : isProcessing
              ? "bg-gray-700 cursor-not-allowed"
              : "bg-blue-600 hover:bg-blue-500 active:scale-95 cursor-pointer shadow-lg shadow-blue-600/20"
          }`}
        >
          <svg
            width="22"
            height="22"
            viewBox="0 0 24 24"
            fill="none"
            stroke="white"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z" />
            <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
            <line x1="12" y1="19" x2="12" y2="23" />
            <line x1="8" y1="23" x2="16" y2="23" />
          </svg>
        </button>
        {!isRecording && !isProcessing && (
          <span className="text-xs text-gray-700 select-none">Click to record</span>
        )}
      </div>
    </div>
  );
}
