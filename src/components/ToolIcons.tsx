import claudeLogo from "../assets/logos/claude.png";
import codexLogo from "../assets/logos/codex.png";
import geminiLogo from "../assets/logos/gemini.png";
import cursorLogo from "../assets/logos/cursor.png";
import windsurfLogo from "../assets/logos/windsurf.png";

interface LogoProps {
  size?: number;
}

export function ClaudeIcon({ size = 18 }: LogoProps) {
  return <img src={claudeLogo} width={size} height={size} alt="Claude" style={{ borderRadius: 3 }} />;
}

export function CodexIcon({ size = 18 }: LogoProps) {
  return <img src={codexLogo} width={size} height={size} alt="Codex" style={{ borderRadius: 3 }} />;
}

export function GeminiIcon({ size = 18 }: LogoProps) {
  return <img src={geminiLogo} width={size} height={size} alt="Gemini" style={{ borderRadius: 3 }} />;
}

export function CursorIcon({ size = 18 }: LogoProps) {
  return <img src={cursorLogo} width={size} height={size} alt="Cursor" style={{ borderRadius: 3 }} />;
}

export function WindsurfIcon({ size = 18 }: LogoProps) {
  return <img src={windsurfLogo} width={size} height={size} alt="Windsurf" style={{ borderRadius: 3 }} />;
}

export function AllToolsIcon({ size = 18 }: LogoProps) {
  return (
    <svg width={size} height={size} viewBox="0 0 24 24" fill="none">
      <rect x="3" y="3" width="8" height="8" rx="2" fill="currentColor" opacity="0.6" />
      <rect x="13" y="3" width="8" height="8" rx="2" fill="currentColor" opacity="0.8" />
      <rect x="3" y="13" width="8" height="8" rx="2" fill="currentColor" opacity="0.8" />
      <rect x="13" y="13" width="8" height="8" rx="2" fill="currentColor" opacity="0.6" />
    </svg>
  );
}
