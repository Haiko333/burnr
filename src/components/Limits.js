import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useEffect, useState, useCallback, useRef } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { RefreshCw } from "lucide-react";
const REFRESH_INTERVAL = 5 * 60 * 1000;
const TOOL_TO_LIMIT_KEY = {
    "claude-code": ["claude"],
    "cursor": ["cursor"],
    "windsurf": ["windsurf"],
};
function Limits({ activeTool }) {
    const { t } = useTranslation();
    const [limits, setLimits] = useState([]);
    const [spinning, setSpinning] = useState(false);
    const intervalRef = useRef(null);
    const fetchLimits = useCallback(async () => {
        try {
            const data = await invoke("get_tool_limits");
            setLimits(data);
        }
        catch {
            // non-critical
        }
    }, []);
    useEffect(() => {
        fetchLimits();
        intervalRef.current = setInterval(fetchLimits, REFRESH_INTERVAL);
        return () => {
            if (intervalRef.current)
                clearInterval(intervalRef.current);
        };
    }, [fetchLimits]);
    const handleRefresh = async () => {
        setSpinning(true);
        await fetchLimits();
        setTimeout(() => setSpinning(false), 600);
    };
    const filteredLimits = activeTool === "all"
        ? limits
        : limits.filter((l) => {
            const keys = TOOL_TO_LIMIT_KEY[activeTool];
            return keys ? keys.some((k) => l.tool.toLowerCase().includes(k)) : false;
        });
    return (_jsxs("div", { className: "limits-section", children: [_jsxs("div", { className: "limits-header", children: [_jsx("span", { className: "limits-title", children: t("sidebar.limits") }), _jsx("button", { className: `limits-refresh ${spinning ? "spinning" : ""}`, onClick: handleRefresh, title: t("limits.refresh"), children: _jsx(RefreshCw, { size: 12 }) })] }), filteredLimits.length === 0 ? (_jsx("div", { className: "limits-empty", children: _jsx("span", { children: t("limits.noData") }) })) : (_jsx("div", { className: "limits-list", children: filteredLimits.map((limit) => (_jsxs("div", { className: "limit-item", children: [_jsxs("div", { className: "limit-info", children: [_jsx("span", { className: "limit-label", children: limit.limitLabel }), _jsxs("span", { className: "limit-pct", children: [limit.currentUsage.toFixed(0), "%"] })] }), _jsx("div", { className: "limit-bar-track", children: _jsx("div", { className: "limit-bar-fill", style: { width: `${Math.min(limit.currentUsage, 100)}%` } }) }), limit.resetTime && (_jsxs("span", { className: "limit-reset", children: [t("limits.resetsIn"), " ", formatResetTime(limit.resetTime)] }))] }, `${limit.tool}-${limit.limitType}`))) }))] }));
}
function formatResetTime(isoTime) {
    const reset = new Date(isoTime);
    const now = new Date();
    const diff = reset.getTime() - now.getTime();
    if (diff <= 0)
        return "now";
    const hours = Math.floor(diff / (1000 * 60 * 60));
    const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
    if (hours > 24) {
        const days = Math.floor(hours / 24);
        return `${days}d ${hours % 24}h`;
    }
    if (hours > 0)
        return `${hours}h ${minutes}m`;
    return `${minutes}m`;
}
export default Limits;
//# sourceMappingURL=Limits.js.map