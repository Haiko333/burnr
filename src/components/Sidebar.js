import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useTranslation } from "react-i18next";
import { Settings } from "lucide-react";
import { TOOLS } from "../toolsConfig";
import Limits from "./Limits";
function Sidebar({ activeTool, onSelectTool, toolAvailability, onOpenSettings }) {
    const { t } = useTranslation();
    const isAvailable = (toolId) => {
        if (toolId === "all")
            return true;
        const entry = toolAvailability.find((ta) => ta.tool === toolId);
        return entry?.available ?? false;
    };
    return (_jsxs("aside", { className: "sidebar", children: [_jsx("nav", { className: "sidebar-nav", children: TOOLS.map((tool) => {
                    const available = isAvailable(tool.id);
                    return (_jsxs("button", { className: `sidebar-item ${activeTool === tool.id ? "active" : ""} ${!available ? "unavailable" : ""}`, onClick: () => onSelectTool(tool.id), children: [_jsx("span", { className: "sidebar-item-icon", children: tool.icon }), _jsx("span", { className: "sidebar-item-label", children: t(tool.labelKey) }), !available && tool.id !== "all" && (_jsx("span", { className: "sidebar-badge", children: t("sidebar.noData") }))] }, tool.id));
                }) }), _jsx("div", { className: "sidebar-limits", children: _jsx(Limits, { activeTool: activeTool }) }), _jsx("div", { className: "sidebar-footer", children: _jsxs("button", { className: "sidebar-item sidebar-settings-btn", onClick: onOpenSettings, children: [_jsx("span", { className: "sidebar-item-icon", children: _jsx(Settings, { size: 18 }) }), _jsx("span", { className: "sidebar-item-label", children: t("sidebar.settings") })] }) })] }));
}
export default Sidebar;
//# sourceMappingURL=Sidebar.js.map