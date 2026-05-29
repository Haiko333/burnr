import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useTranslation } from "react-i18next";
import { Brain, Calendar, Trophy, Zap } from "lucide-react";
import { formatTokens, formatCost } from "../utils/format";
function calculateStreaks(dailyUsage) {
    const activeDates = new Set(dailyUsage.filter((d) => d.costUsd > 0).map((d) => d.date));
    const today = new Date();
    let current = 0;
    const cursor = new Date(today);
    while (true) {
        const dateStr = cursor.toISOString().slice(0, 10);
        if (activeDates.has(dateStr)) {
            current++;
            cursor.setDate(cursor.getDate() - 1);
        }
        else {
            break;
        }
    }
    const sortedDates = Array.from(activeDates).sort();
    let longest = 0;
    let streak = 0;
    let prevDate = null;
    for (const dateStr of sortedDates) {
        const date = new Date(dateStr);
        if (prevDate) {
            const diff = (date.getTime() - prevDate.getTime()) / (1000 * 60 * 60 * 24);
            if (diff === 1) {
                streak++;
            }
            else {
                longest = Math.max(longest, streak);
                streak = 1;
            }
        }
        else {
            streak = 1;
        }
        prevDate = date;
    }
    longest = Math.max(longest, streak);
    return { current, longest };
}
function getRecent30dStats(dailyUsage) {
    const today = new Date();
    const thirtyDaysAgo = new Date(today);
    thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);
    const cutoff = thirtyDaysAgo.toISOString().slice(0, 10);
    const recent = dailyUsage.filter((d) => d.date >= cutoff);
    const cost = recent.reduce((sum, d) => sum + d.costUsd, 0);
    const days = recent.filter((d) => d.costUsd > 0).length;
    return { cost, days };
}
function StatCards({ dailyUsage, models, costIsEstimated }) {
    const { t } = useTranslation();
    const streaks = calculateStreaks(dailyUsage);
    const recent30d = getRecent30dStats(dailyUsage);
    const topModel = models.length > 0 ? models.reduce((a, b) => (a.entryCount > b.entryCount ? a : b)) : null;
    return (_jsxs("div", { className: "stat-cards", children: [_jsxs("div", { className: "stat-card", children: [_jsx("div", { className: "stat-card-icon-wrap purple", children: _jsx(Brain, { size: 20 }) }), _jsxs("div", { className: "stat-card-content", children: [_jsx("span", { className: "stat-card-value", children: topModel ? topModel.model.replace("claude-", "").replace(/-\d.*$/, "") : "—" }), _jsx("span", { className: "stat-card-label", children: t("stats.mostUsedModel") }), _jsx("span", { className: "stat-card-detail", children: topModel ? `${formatTokens(topModel.entryCount)} ${t("stats.calls")}` : "" })] })] }), _jsxs("div", { className: "stat-card", children: [_jsx("div", { className: "stat-card-icon-wrap purple-light", children: _jsx(Calendar, { size: 20 }) }), _jsxs("div", { className: "stat-card-content", children: [_jsx("span", { className: "stat-card-value", children: formatCost(recent30d.cost, costIsEstimated) }), _jsx("span", { className: "stat-card-label", children: t("stats.last30Days") }), _jsxs("span", { className: "stat-card-detail", children: [recent30d.days, " ", t("stats.activeDays")] })] })] }), _jsxs("div", { className: "stat-card", children: [_jsx("div", { className: "stat-card-icon-wrap purple-muted", children: _jsx(Trophy, { size: 20 }) }), _jsxs("div", { className: "stat-card-content", children: [_jsxs("span", { className: "stat-card-value", children: [streaks.longest, "d"] }), _jsx("span", { className: "stat-card-label", children: t("stats.longestStreak") })] })] }), _jsxs("div", { className: "stat-card", children: [_jsx("div", { className: "stat-card-icon-wrap purple-bright", children: _jsx(Zap, { size: 20 }) }), _jsxs("div", { className: "stat-card-content", children: [_jsxs("span", { className: "stat-card-value", children: [streaks.current, "d"] }), _jsx("span", { className: "stat-card-label", children: t("stats.currentStreak") })] })] })] }));
}
export default StatCards;
//# sourceMappingURL=StatCards.js.map