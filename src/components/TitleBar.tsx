import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Square, X } from "lucide-react";
import BurnrLogo from "./BurnrLogo";

function TitleBar() {
  const appWindow = getCurrentWindow();

  const handleDragStart = (e: React.MouseEvent) => {
    if ((e.target as HTMLElement).closest(".title-bar-controls")) return;
    e.preventDefault();
    appWindow.startDragging();
  };

  return (
    <div className="title-bar" onMouseDown={handleDragStart}>
      <div className="title-bar-brand">
        <BurnrLogo size={16} />
      </div>
      <div className="title-bar-controls">
        <button className="title-bar-btn" onClick={() => appWindow.minimize()}>
          <Minus size={14} />
        </button>
        <button className="title-bar-btn" onClick={() => appWindow.toggleMaximize()}>
          <Square size={11} />
        </button>
        <button className="title-bar-btn title-bar-close" onClick={() => appWindow.close()}>
          <X size={14} />
        </button>
      </div>
    </div>
  );
}

export default TitleBar;
