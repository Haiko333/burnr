import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import claudeLogo from "../assets/logos/claude.png";
import codexLogo from "../assets/logos/codex.png";
import geminiLogo from "../assets/logos/gemini.png";
import cursorLogo from "../assets/logos/cursor.png";
import windsurfLogo from "../assets/logos/windsurf.png";
export function ClaudeIcon({ size = 18 }) {
    return _jsx("img", { src: claudeLogo, width: size, height: size, alt: "Claude", style: { borderRadius: 3 } });
}
export function CodexIcon({ size = 18 }) {
    return _jsx("img", { src: codexLogo, width: size, height: size, alt: "Codex", style: { borderRadius: 3 } });
}
export function GeminiIcon({ size = 18 }) {
    return _jsx("img", { src: geminiLogo, width: size, height: size, alt: "Gemini", style: { borderRadius: 3 } });
}
export function CursorIcon({ size = 18 }) {
    return _jsx("img", { src: cursorLogo, width: size, height: size, alt: "Cursor", style: { borderRadius: 3 } });
}
export function WindsurfIcon({ size = 18 }) {
    return _jsx("img", { src: windsurfLogo, width: size, height: size, alt: "Windsurf", style: { borderRadius: 3 } });
}
export function AllToolsIcon({ size = 18 }) {
    return (_jsxs("svg", { width: size, height: size, viewBox: "0 0 24 24", fill: "none", children: [_jsx("rect", { x: "3", y: "3", width: "8", height: "8", rx: "2", fill: "currentColor", opacity: "0.6" }), _jsx("rect", { x: "13", y: "3", width: "8", height: "8", rx: "2", fill: "currentColor", opacity: "0.8" }), _jsx("rect", { x: "3", y: "13", width: "8", height: "8", rx: "2", fill: "currentColor", opacity: "0.8" }), _jsx("rect", { x: "13", y: "13", width: "8", height: "8", rx: "2", fill: "currentColor", opacity: "0.6" })] }));
}
//# sourceMappingURL=ToolIcons.js.map