import { useTranslation } from "react-i18next";
import { Brain, Calendar, Trophy, Zap } from "lucide-react";
import { formatTokens, formatCost } from "../utils/format";
import { DailyUsage, ModelStats } from "../types";

interface StatCardsProps {
  dailyUsage: DailyUsage[];
  models: ModelStats[];
  costIsEstimated: boolean;
}

function calculateStreaks(dailyUsage: DailyUsage[]): { current: number; longest: number } {
  const activeDates = new Set(dailyUsage.filter((d) => d.costUsd > 0).map((d) => d.date));

  const today = new Date();
  let current = 0;
  const cursor = new Date(today);

  while (true) {
    const dateStr = cursor.toISOString().slice(0, 10);
    if (activeDates.has(dateStr)) {
      current++;
      cursor.setDate(cursor.getDate() - 1);
    } else {
      break;
    }
  }

  const sortedDates = Array.from(activeDates).sort();
  let longest = 0;
  let streak = 0;
  let prevDate: Date | null = null;

  for (const dateStr of sortedDates) {
    const date = new Date(dateStr);
    if (prevDate) {
      const diff = (date.getTime() - prevDate.getTime()) / (1000 * 60 * 60 * 24);
      if (diff === 1) {
        streak++;
      } else {
        longest = Math.max(longest, streak);
        streak = 1;
      }
    } else {
      streak = 1;
    }
    prevDate = date;
  }
  longest = Math.max(longest, streak);

  return { current, longest };
}

function getRecent30dStats(dailyUsage: DailyUsage[]): { cost: number; days: number } {
  const today = new Date();
  const thirtyDaysAgo = new Date(today);
  thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);
  const cutoff = thirtyDaysAgo.toISOString().slice(0, 10);

  const recent = dailyUsage.filter((d) => d.date >= cutoff);
  const cost = recent.reduce((sum, d) => sum + d.costUsd, 0);
  const days = recent.filter((d) => d.costUsd > 0).length;

  return { cost, days };
}

function StatCards({ dailyUsage, models, costIsEstimated }: StatCardsProps) {
  const { t } = useTranslation();
  const streaks = calculateStreaks(dailyUsage);
  const recent30d = getRecent30dStats(dailyUsage);
  const topModel = models.length > 0 ? models.reduce((a, b) => (a.entryCount > b.entryCount ? a : b)) : null;

  return (
    <div className="stat-cards">
      <div className="stat-card">
        <div className="stat-card-icon-wrap purple">
          <Brain size={20} />
        </div>
        <div className="stat-card-content">
          <span className="stat-card-value">
            {topModel ? topModel.model.replace("claude-", "").replace(/-\d.*$/, "") : "—"}
          </span>
          <span className="stat-card-label">{t("stats.mostUsedModel")}</span>
          <span className="stat-card-detail">
            {topModel ? `${formatTokens(topModel.entryCount)} ${t("stats.calls")}` : ""}
          </span>
        </div>
      </div>

      <div className="stat-card">
        <div className="stat-card-icon-wrap purple-light">
          <Calendar size={20} />
        </div>
        <div className="stat-card-content">
          <span className="stat-card-value">{formatCost(recent30d.cost, costIsEstimated)}</span>
          <span className="stat-card-label">{t("stats.last30Days")}</span>
          <span className="stat-card-detail">{recent30d.days} {t("stats.activeDays")}</span>
        </div>
      </div>

      <div className="stat-card">
        <div className="stat-card-icon-wrap purple-muted">
          <Trophy size={20} />
        </div>
        <div className="stat-card-content">
          <span className="stat-card-value">{streaks.longest}d</span>
          <span className="stat-card-label">{t("stats.longestStreak")}</span>
        </div>
      </div>

      <div className="stat-card">
        <div className="stat-card-icon-wrap purple-bright">
          <Zap size={20} />
        </div>
        <div className="stat-card-content">
          <span className="stat-card-value">{streaks.current}d</span>
          <span className="stat-card-label">{t("stats.currentStreak")}</span>
        </div>
      </div>
    </div>
  );
}

export default StatCards;
