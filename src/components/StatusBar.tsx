import { useAppStore } from "../stores/appStore";

export function StatusBar() {
  const status = useAppStore((s) => s.status);
  const statusMessage = useAppStore((s) => s.statusMessage);
  const lastTranscript = useAppStore((s) => s.lastTranscript);
  const apiKeySet = useAppStore((s) => s.apiKeySet);

  return (
    <div className="border-b border-gray-800 px-6 py-3 flex items-center justify-between">
      <div className="flex items-center gap-3">
        <h2 className="font-semibold text-lg">VoiceFlow</h2>
        <StatusDot status={status} />
        {statusMessage && (
          <span className="text-sm text-gray-400">{statusMessage}</span>
        )}
      </div>
      <div className="flex items-center gap-4 text-sm">
        {!apiKeySet && (
          <span className="text-yellow-400">API key not set</span>
        )}
        {lastTranscript && (
          <span className="text-gray-500 max-w-xs truncate">
            Last: {lastTranscript}
          </span>
        )}
      </div>
    </div>
  );
}

function StatusDot({ status }: { status: string }) {
  const colors: Record<string, string> = {
    idle: "bg-gray-500",
    recording: "bg-red-500 animate-pulse",
    processing: "bg-blue-500 animate-pulse",
    done: "bg-green-500",
    error: "bg-orange-500",
  };

  return (
    <span className={`inline-block w-2.5 h-2.5 rounded-full ${colors[status] ?? "bg-gray-500"}`} />
  );
}
