import { useTranslation } from "react-i18next";
import { DailyUsage } from "../types";

interface HeatmapProps {
  dailyUsage: DailyUsage[];
}

function getIntensityLevel(cost: number, maxCost: number): number {
  if (cost === 0) return 0;
  const ratio = cost / maxCost;
  if (ratio < 0.15) return 1;
  if (ratio < 0.35) return 2;
  if (ratio < 0.6) return 3;
  return 4;
}

function getWeeksArray(dailyUsage: DailyUsage[]): { date: string; cost: number }[][] {
  const today = new Date();
  const startDate = new Date(today);
  startDate.setDate(startDate.getDate() - 364);
  startDate.setDate(startDate.getDate() - startDate.getDay());

  const usageMap = new Map<string, number>();
  for (const d of dailyUsage) {
    usageMap.set(d.date, d.costUsd);
  }

  const weeks: { date: string; cost: number }[][] = [];
  let currentWeek: { date: string; cost: number }[] = [];
  const cursor = new Date(startDate);

  while (cursor <= today) {
    const dateStr = cursor.toISOString().slice(0, 10);
    currentWeek.push({ date: dateStr, cost: usageMap.get(dateStr) ?? 0 });

    if (currentWeek.length === 7) {
      weeks.push(currentWeek);
      currentWeek = [];
    }
    cursor.setDate(cursor.getDate() + 1);
  }

  if (currentWeek.length > 0) {
    weeks.push(currentWeek);
  }

  return weeks;
}

function getMonthLabels(weeks: { date: string; cost: number }[][], months: string[]): { label: string; col: number }[] {
  const labels: { label: string; col: number }[] = [];
  let lastMonth = -1;

  for (let i = 0; i < weeks.length; i++) {
    const firstDay = weeks[i][0];
    if (!firstDay) continue;
    const month = new Date(firstDay.date).getMonth();
    if (month !== lastMonth) {
      labels.push({ label: months[month], col: i });
      lastMonth = month;
    }
  }

  return labels;
}

function Heatmap({ dailyUsage }: HeatmapProps) {
  const { t } = useTranslation();
  const weeks = getWeeksArray(dailyUsage);
  const maxCost = Math.max(...dailyUsage.map((d) => d.costUsd), 1);
  const months = t("heatmap.months", { returnObjects: true }) as string[];
  const days = t("heatmap.days", { returnObjects: true }) as string[];
  const monthLabels = getMonthLabels(weeks, months);

  return (
    <div className="heatmap-container">
      <div className="heatmap-wrapper">
        <div className="heatmap-months" style={{ gridTemplateColumns: `repeat(${weeks.length}, 1fr)` }}>
          {monthLabels.map((m) => (
            <span key={`${m.label}-${m.col}`} style={{ gridColumn: m.col + 1 }}>
              {m.label}
            </span>
          ))}
        </div>
        <div className="heatmap-body">
          <div className="heatmap-days">
            {days.map((d, i) => (
              <span key={i}>{d}</span>
            ))}
          </div>
          <div className="heatmap-grid" style={{ gridTemplateColumns: `repeat(${weeks.length}, 1fr)` }}>
            {weeks.map((week, wi) =>
              week.map((day, di) => (
                <div
                  key={`${wi}-${di}`}
                  className={`heatmap-cell level-${getIntensityLevel(day.cost, maxCost)}`}
                  title={`${day.date}: $${day.cost.toFixed(2)}`}
                  style={{ gridColumn: wi + 1, gridRow: di + 1 }}
                />
              ))
            )}
          </div>
        </div>
        <div className="heatmap-legend">
          <span>{t("heatmap.less")}</span>
          <div className="heatmap-cell level-0" />
          <div className="heatmap-cell level-1" />
          <div className="heatmap-cell level-2" />
          <div className="heatmap-cell level-3" />
          <div className="heatmap-cell level-4" />
          <span>{t("heatmap.more")}</span>
        </div>
      </div>
    </div>
  );
}

export default Heatmap;
