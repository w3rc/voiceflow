import type { ContextInfo } from "./tauriCommands";

export function getContextLabel(info: ContextInfo): string {
  const process = info.process_name.toLowerCase();
  const title = info.window_title.toLowerCase();

  if (
    process.includes("slack") ||
    process.includes("discord") ||
    process.includes("telegram")
  ) {
    return "Chat";
  }
  if (
    process.includes("outlook") ||
    process.includes("thunderbird") ||
    title.includes("mail")
  ) {
    return "Email";
  }
  if (
    process.includes("code") ||
    process.includes("vim") ||
    process.includes("idea")
  ) {
    return "Code Editor";
  }
  if (
    process.includes("docs") ||
    process.includes("word") ||
    process.includes("notion")
  ) {
    return "Document";
  }
  return "General";
}
