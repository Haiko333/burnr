import { useEffect, useState, useCallback, useRef } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { RefreshCw } from "lucide-react";
import { Tool, ToolLimits } from "../types";

const REFRESH_INTERVAL = 5 * 60 * 1000;

const TOOL_TO_LIMIT_KEY: Record<string, string[]> = {
  "claude-code": ["claude"],
  "cursor": ["cursor"],
  "windsurf": ["windsurf"],
};

interface LimitsProps {
  activeTool: Tool;
}

function Limits({ activeTool }: LimitsProps) {
  const { t } = useTranslation();
  const [limits, setLimits] = useState<ToolLimits[]>([]);
  const [spinning, setSpinning] = useState(false);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const fetchLimits = useCallback(async () => {
    try {
      const data = await invoke<ToolLimits[]>("get_tool_limits");
      setLimits(data);
    } catch {
      // non-critical
    }
  }, []);

  useEffect(() => {
    fetchLimits();
    intervalRef.current = setInterval(fetchLimits, REFRESH_INTERVAL);
    return () => {
      if (intervalRef.current) clearInterval(intervalRef.current);
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

  return (
    <div className="limits-section">
      <div className="limits-header">
        <span className="limits-title">{t("sidebar.limits")}</span>
        <button className={`limits-refresh ${spinning ? "spinning" : ""}`} onClick={handleRefresh} title={t("limits.refresh")}>
          <RefreshCw size={12} />
        </button>
      </div>
      {filteredLimits.length === 0 ? (
        <div className="limits-empty">
          <span>{t("limits.noData")}</span>
        </div>
      ) : (
        <div className="limits-list">
          {filteredLimits.map((limit) => (
            <div key={`${limit.tool}-${limit.limitType}`} className="limit-item">
              <div className="limit-info">
                <span className="limit-label">{limit.limitLabel}</span>
                <span className="limit-pct">{limit.currentUsage.toFixed(0)}%</span>
              </div>
              <div className="limit-bar-track">
                <div
                  className="limit-bar-fill"
                  style={{ width: `${Math.min(limit.currentUsage, 100)}%` }}
                />
              </div>
              {limit.resetTime && (
                <span className="limit-reset">
                  {t("limits.resetsIn")} {formatResetTime(limit.resetTime)}
                </span>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

function formatResetTime(isoTime: string): string {
  const reset = new Date(isoTime);
  const now = new Date();
  const diff = reset.getTime() - now.getTime();
  if (diff <= 0) return "now";

  const hours = Math.floor(diff / (1000 * 60 * 60));
  const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));

  if (hours > 24) {
    const days = Math.floor(hours / 24);
    return `${days}d ${hours % 24}h`;
  }
  if (hours > 0) return `${hours}h ${minutes}m`;
  return `${minutes}m`;
}

export default Limits;
