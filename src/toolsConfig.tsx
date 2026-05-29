import { ToolConfig } from "./types";
import { AllToolsIcon, ClaudeIcon, CodexIcon, GeminiIcon, CursorIcon, WindsurfIcon } from "./components/ToolIcons";

export const TOOLS: ToolConfig[] = [
  { id: "all", labelKey: "tools.all", icon: <AllToolsIcon /> },
  { id: "claude-code", labelKey: "tools.claudeCode", icon: <ClaudeIcon /> },
  { id: "codex", labelKey: "tools.codex", icon: <CodexIcon /> },
  { id: "gemini", labelKey: "tools.gemini", icon: <GeminiIcon /> },
  { id: "cursor", labelKey: "tools.cursor", icon: <CursorIcon /> },
  { id: "windsurf", labelKey: "tools.windsurf", icon: <WindsurfIcon /> },
];
