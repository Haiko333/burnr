import { useEffect, useState, useCallback, useRef } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { Tool, GlobalStats, ModelStats, ProjectStats, ToolAvailability } from "./types";
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

function aggregateModels(projects: ProjectStats[]): ModelStats[] {
  const map = new Map<string, ModelStats>();
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
      } else {
        map.set(m.model, { ...m });
      }
    }
  }
  return Array.from(map.values());
}

function App() {
  const { t } = useTranslation();
  const { theme, setTheme } = useTheme();
  const [stats, setStats] = useState<GlobalStats | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [initialLoading, setInitialLoading] = useState(true);
  const [refreshing, setRefreshing] = useState(false);
  const [activeTool, setActiveTool] = useState<Tool>("claude-code");
  const [toolAvailability, setToolAvailability] = useState<ToolAvailability[]>([]);
  const [contentKey, setContentKey] = useState(0);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const prevToolRef = useRef<Tool>(activeTool);

  const fetchStats = useCallback(async (tool: Tool, isInitial: boolean) => {
    if (isInitial) {
      setInitialLoading(true);
    } else {
      setRefreshing(true);
    }
    setError(null);
    try {
      const toolArg = tool === "all" ? null : tool;
      const data = await invoke<GlobalStats>("get_all_stats", {
        billingType: "subscription",
        tool: toolArg,
      });
      setStats(data);
    } catch (err) {
      setError(String(err));
    } finally {
      setInitialLoading(false);
      setRefreshing(false);
    }
  }, []);

  const fetchAvailability = useCallback(async () => {
    try {
      const availability = await invoke<ToolAvailability[]>("get_available_tools");
      setToolAvailability(availability);
    } catch {
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

  return (
    <div className="app-layout">
      <ResizeHandles />
      <TitleBar />
      <Sidebar
        activeTool={activeTool}
        onSelectTool={setActiveTool}
        toolAvailability={toolAvailability}
        onOpenSettings={() => setSettingsOpen(true)}
      />
      <main className="main-content">
        {refreshing && <div className="refresh-indicator" />}

        {initialLoading && !stats && <SkeletonDashboard />}

        {error && !initialLoading && (
          <div className="error-state">
            <div className="error-icon">
              <span>!</span>
            </div>
            <h2>{t("error.somethingWrong")}</h2>
            <p className="error-message">{error}</p>
            <button className="retry-btn" onClick={() => fetchStats(activeTool, true)}>
              {t("error.retry")}
            </button>
          </div>
        )}

        {!initialLoading && !error && !hasData && (
          <div className="empty-state">
            <div className="empty-state-graphic">
              <div className="empty-state-ring" />
              <div className="empty-state-icon">
                <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
                  <circle cx="12" cy="12" r="8" stroke="currentColor" strokeWidth="2" opacity="0.5" />
                  <circle cx="12" cy="12" r="3" fill="currentColor" opacity="0.7" />
                </svg>
              </div>
            </div>
            <h2>{t("empty.noDataYet")}</h2>
            <p>{t(`empty.${activeTool === "claude-code" ? "claudeCode" : activeTool}`)}</p>
            <button className="empty-state-refresh" onClick={handleRefresh}>
              <RefreshCw size={14} />
              {t("limits.refresh")}
            </button>
            <span className="empty-state-hint">{t("empty.autoRefresh")}</span>
          </div>
        )}

        {!initialLoading && !error && hasData && (
          <div className="dashboard-content" key={contentKey}>
            <Header
              totalCostUsd={stats!.totalCostUsd}
              costIsEstimated={stats!.costIsEstimated}
              totalInputTokens={stats!.totalInputTokens}
              totalOutputTokens={stats!.totalOutputTokens}
              totalCacheReadTokens={stats!.totalCacheReadTokens}
              totalCacheCreationTokens={stats!.totalCacheCreationTokens}
              totalSessions={stats!.totalSessions}
              totalEntries={stats!.totalEntries}
              onRefresh={handleRefresh}
              refreshing={refreshing}
            />
            <Heatmap dailyUsage={stats!.dailyUsage} />
            <StatCards
              dailyUsage={stats!.dailyUsage}
              models={models}
              costIsEstimated={stats!.costIsEstimated}
            />
            <ModelTable models={models} costIsEstimated={stats!.costIsEstimated} />
          </div>
        )}
      </main>

      <Settings
        isOpen={settingsOpen}
        onClose={() => setSettingsOpen(false)}
        theme={theme}
        onThemeChange={setTheme}
        stats={stats}
      />
    </div>
  );
}

export default App;
