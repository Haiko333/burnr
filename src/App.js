import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useEffect, useState, useCallback, useRef } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { RefreshCw } from "lucide-react";
import { useTheme } from "./hooks/useTheme";
import TitleBar from "./components/TitleBar";
import ResizeHandles from "./components/ResizeHandles";
import Sidebar from "./components/Sidebar";
import Header from "./components/Header";
import Heatmap from "./components/Heatmap";
import StatCards from "./components/StatCards";
import ModelTable from "./components/ModelTable";
import Settings from "./components/Settings";
import SkeletonDashboard from "./components/Skeleton";
function aggregateModels(projects) {
    const map = new Map();
    for (const p of projects) {
        for (const m of p.modelsUsed) {
            const existing = map.get(m.model);
            if (existing) {
                existing.inputTokens += m.inputTokens;
                existing.outputTokens += m.outputTokens;
                existing.cacheCreationTokens += m.cacheCreationTokens;
                existing.cacheReadTokens += m.cacheReadTokens;
                existing.costUsd += m.costUsd;
                existing.entryCount += m.entryCount;
            }
            else {
                map.set(m.model, { ...m });
            }
        }
    }
    return Array.from(map.values());
}
function App() {
    const { t } = useTranslation();
    const { theme, setTheme } = useTheme();
    const [stats, setStats] = useState(null);
    const [error, setError] = useState(null);
    const [initialLoading, setInitialLoading] = useState(true);
    const [refreshing, setRefreshing] = useState(false);
    const [activeTool, setActiveTool] = useState("claude-code");
    const [toolAvailability, setToolAvailability] = useState([]);
    const [contentKey, setContentKey] = useState(0);
    const [settingsOpen, setSettingsOpen] = useState(false);
    const prevToolRef = useRef(activeTool);
    const fetchStats = useCallback(async (tool, isInitial) => {
        if (isInitial) {
            setInitialLoading(true);
        }
        else {
            setRefreshing(true);
        }
        setError(null);
        try {
            const toolArg = tool === "all" ? null : tool;
            const data = await invoke("get_all_stats", {
                billingType: "subscription",
                tool: toolArg,
            });
            setStats(data);
        }
        catch (err) {
            setError(String(err));
        }
        finally {
            setInitialLoading(false);
            setRefreshing(false);
        }
    }, []);
    const fetchAvailability = useCallback(async () => {
        try {
            const availability = await invoke("get_available_tools");
            setToolAvailability(availability);
        }
        catch {
            // non-critical
        }
    }, []);
    useEffect(() => {
        const isToolSwitch = prevToolRef.current !== activeTool;
        prevToolRef.current = activeTool;
        if (isToolSwitch) {
            setContentKey((k) => k + 1);
        }
        fetchStats(activeTool, !stats);
        fetchAvailability();
    }, [activeTool, fetchStats, fetchAvailability]);
    useEffect(() => {
        const interval = setInterval(() => {
            fetchStats(activeTool, false);
            fetchAvailability();
        }, 5 * 60 * 1000);
        return () => clearInterval(interval);
    }, [activeTool, fetchStats, fetchAvailability]);
    const handleRefresh = () => {
        fetchStats(activeTool, false);
        fetchAvailability();
    };
    const hasData = stats && stats.totalEntries > 0;
    const models = stats ? aggregateModels(stats.projects) : [];
    return (_jsxs("div", { className: "app-layout", children: [_jsx(ResizeHandles, {}), _jsx(TitleBar, {}), _jsx(Sidebar, { activeTool: activeTool, onSelectTool: setActiveTool, toolAvailability: toolAvailability, onOpenSettings: () => setSettingsOpen(true) }), _jsxs("main", { className: "main-content", children: [refreshing && _jsx("div", { className: "refresh-indicator" }), initialLoading && !stats && _jsx(SkeletonDashboard, {}), error && !initialLoading && (_jsxs("div", { className: "error-state", children: [_jsx("div", { className: "error-icon", children: _jsx("span", { children: "!" }) }), _jsx("h2", { children: t("error.somethingWrong") }), _jsx("p", { className: "error-message", children: error }), _jsx("button", { className: "retry-btn", onClick: () => fetchStats(activeTool, true), children: t("error.retry") })] })), !initialLoading && !error && !hasData && (_jsxs("div", { className: "empty-state", children: [_jsxs("div", { className: "empty-state-graphic", children: [_jsx("div", { className: "empty-state-ring" }), _jsx("div", { className: "empty-state-icon", children: _jsxs("svg", { width: "32", height: "32", viewBox: "0 0 24 24", fill: "none", children: [_jsx("circle", { cx: "12", cy: "12", r: "8", stroke: "currentColor", strokeWidth: "2", opacity: "0.5" }), _jsx("circle", { cx: "12", cy: "12", r: "3", fill: "currentColor", opacity: "0.7" })] }) })] }), _jsx("h2", { children: t("empty.noDataYet") }), _jsx("p", { children: t(`empty.${activeTool === "claude-code" ? "claudeCode" : activeTool}`) }), _jsxs("button", { className: "empty-state-refresh", onClick: handleRefresh, children: [_jsx(RefreshCw, { size: 14 }), t("limits.refresh")] }), _jsx("span", { className: "empty-state-hint", children: t("empty.autoRefresh") })] })), !initialLoading && !error && hasData && (_jsxs("div", { className: "dashboard-content", children: [_jsx(Header, { totalCostUsd: stats.totalCostUsd, costIsEstimated: stats.costIsEstimated, totalInputTokens: stats.totalInputTokens, totalOutputTokens: stats.totalOutputTokens, totalCacheReadTokens: stats.totalCacheReadTokens, totalCacheCreationTokens: stats.totalCacheCreationTokens, totalSessions: stats.totalSessions, totalEntries: stats.totalEntries, onRefresh: handleRefresh, refreshing: refreshing }), _jsx(Heatmap, { dailyUsage: stats.dailyUsage }), _jsx(StatCards, { dailyUsage: stats.dailyUsage, models: models, costIsEstimated: stats.costIsEstimated }), _jsx(ModelTable, { models: models, costIsEstimated: stats.costIsEstimated })] }, contentKey))] }), _jsx(Settings, { isOpen: settingsOpen, onClose: () => setSettingsOpen(false), theme: theme, onThemeChange: setTheme, stats: stats })] }));
}
export default App;
//# sourceMappingURL=App.js.map