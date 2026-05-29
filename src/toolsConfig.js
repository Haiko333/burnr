import { jsx as _jsx } from "react/jsx-runtime";
import { AllToolsIcon, ClaudeIcon, CodexIcon, GeminiIcon, CursorIcon, WindsurfIcon } from "./components/ToolIcons";
export const TOOLS = [
    { id: "all", labelKey: "tools.all", icon: _jsx(AllToolsIcon, {}) },
    { id: "claude-code", labelKey: "tools.claudeCode", icon: _jsx(ClaudeIcon, {}) },
    { id: "codex", labelKey: "tools.codex", icon: _jsx(CodexIcon, {}) },
    { id: "gemini", labelKey: "tools.gemini", icon: _jsx(GeminiIcon, {}) },
    { id: "cursor", labelKey: "tools.cursor", icon: _jsx(CursorIcon, {}) },
    { id: "windsurf", labelKey: "tools.windsurf", icon: _jsx(WindsurfIcon, {}) },
];
//# sourceMappingURL=toolsConfig.js.map