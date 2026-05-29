import { useTranslation } from "react-i18next";
import { RefreshCw } from "lucide-react";
import { formatTokens, formatCost } from "../utils/format";

interface HeaderProps {
  totalCostUsd: number;
  costIsEstimated: boolean;
  totalInputTokens: number;
  totalOutputTokens: number;
  totalCacheReadTokens: number;
  totalCacheCreationTokens: number;
  totalSessions: number;
  totalEntries: number;
  onRefresh: () => void;
  refreshing: boolean;
}

function Header({
  totalCostUsd,
  costIsEstimated,
  totalInputTokens,
  totalOutputTokens,
  totalCacheReadTokens,
  totalCacheCreationTokens,
  totalSessions,
  totalEntries,
  onRefresh,
  refreshing,
}: HeaderProps) {
  const { t } = useTranslation();
  const totalTokens =
    totalInputTokens + totalOutputTokens + totalCacheReadTokens + totalCacheCreationTokens;

  return (
    <header className="dashboard-header">
      <div className="header-stats">
        <div className="header-stat">
          <span className="header-stat-value">{formatTokens(totalInputTokens)}</span>
          <span className="header-stat-label">{t("header.input")}</span>
        </div>
        <div className="header-stat">
          <span className="header-stat-value">{formatTokens(totalOutputTokens)}</span>
          <span className="header-stat-label">{t("header.output")}</span>
        </div>
        <div className="header-stat">
          <span className="header-stat-value">{formatTokens(totalCacheReadTokens + totalCacheCreationTokens)}</span>
          <span className="header-stat-label">{t("header.cache")}</span>
        </div>
        <div className="header-stat">
          <span className="header-stat-value">{totalSessions}</span>
          <span className="header-stat-label">{t("header.sessions")}</span>
        </div>
        <div className="header-stat">
          <span className="header-stat-value">{formatTokens(totalTokens)}</span>
          <span className="header-stat-label">{t("header.total")}</span>
        </div>
        <div className="header-stat primary">
          <span className="header-stat-value">{formatCost(totalCostUsd, costIsEstimated)}</span>
          <span className="header-stat-label">{t("header.cost")}</span>
        </div>
      </div>
      <button
        className={`header-refresh ${refreshing ? "spinning" : ""}`}
        onClick={onRefresh}
        title={t("limits.refresh")}
      >
        <RefreshCw size={16} />
      </button>
    </header>
  );
}

export default Header;
