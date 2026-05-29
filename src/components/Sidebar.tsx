import { useTranslation } from "react-i18next";
import { Settings } from "lucide-react";
import { Tool, ToolAvailability } from "../types";
import { TOOLS } from "../toolsConfig";
import Limits from "./Limits";

interface SidebarProps {
  activeTool: Tool;
  onSelectTool: (tool: Tool) => void;
  toolAvailability: ToolAvailability[];
  onOpenSettings: () => void;
}

function Sidebar({ activeTool, onSelectTool, toolAvailability, onOpenSettings }: SidebarProps) {
  const { t } = useTranslation();

  const isAvailable = (toolId: Tool): boolean => {
    if (toolId === "all") return true;
    const entry = toolAvailability.find((ta) => ta.tool === toolId);
    return entry?.available ?? false;
  };

  return (
    <aside className="sidebar">
      <nav className="sidebar-nav">
        {TOOLS.map((tool) => {
          const available = isAvailable(tool.id);
          return (
            <button
              key={tool.id}
              className={`sidebar-item ${activeTool === tool.id ? "active" : ""} ${!available ? "unavailable" : ""}`}
              onClick={() => onSelectTool(tool.id)}
            >
              <span className="sidebar-item-icon">{tool.icon}</span>
              <span className="sidebar-item-label">{t(tool.labelKey)}</span>
            </button>
          );
        })}
      </nav>

      <div className="sidebar-limits">
        <Limits activeTool={activeTool} />
      </div>

      <div className="sidebar-footer">
        <button className="sidebar-item sidebar-settings-btn" onClick={onOpenSettings}>
          <span className="sidebar-item-icon"><Settings size={18} /></span>
          <span className="sidebar-item-label">{t("sidebar.settings")}</span>
        </button>
      </div>
    </aside>
  );
}

export default Sidebar;
