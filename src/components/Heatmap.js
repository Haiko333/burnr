import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useTranslation } from "react-i18next";
function getIntensityLevel(cost, maxCost) {
    if (cost === 0)
        return 0;
    const ratio = cost / maxCost;
    if (ratio < 0.15)
        return 1;
    if (ratio < 0.35)
        return 2;
    if (ratio < 0.6)
        return 3;
    return 4;
}
function getWeeksArray(dailyUsage) {
    const today = new Date();
    const startDate = new Date(today);
    startDate.setDate(startDate.getDate() - 364);
    startDate.setDate(startDate.getDate() - startDate.getDay());
    const usageMap = new Map();
    for (const d of dailyUsage) {
        usageMap.set(d.date, d.costUsd);
    }
    const weeks = [];
    let currentWeek = [];
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
function getMonthLabels(weeks) {
    const labels = [];
    const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let lastMonth = -1;
    for (let i = 0; i < weeks.length; i++) {
        const firstDay = weeks[i][0];
        if (!firstDay)
            continue;
        const month = new Date(firstDay.date).getMonth();
        if (month !== lastMonth) {
            labels.push({ label: months[month], col: i });
            lastMonth = month;
        }
    }
    return labels;
}
function Heatmap({ dailyUsage }) {
    const { t } = useTranslation();
    const weeks = getWeeksArray(dailyUsage);
    const maxCost = Math.max(...dailyUsage.map((d) => d.costUsd), 1);
    const monthLabels = getMonthLabels(weeks);
    return (_jsx("div", { className: "heatmap-container", children: _jsxs("div", { className: "heatmap-wrapper", children: [_jsx("div", { className: "heatmap-months", style: { gridTemplateColumns: `repeat(${weeks.length}, 1fr)` }, children: monthLabels.map((m) => (_jsx("span", { style: { gridColumn: m.col + 1 }, children: m.label }, `${m.label}-${m.col}`))) }), _jsxs("div", { className: "heatmap-body", children: [_jsxs("div", { className: "heatmap-days", children: [_jsx("span", {}), _jsx("span", { children: "Mon" }), _jsx("span", {}), _jsx("span", { children: "Wed" }), _jsx("span", {}), _jsx("span", { children: "Fri" }), _jsx("span", {})] }), _jsx("div", { className: "heatmap-grid", style: { gridTemplateColumns: `repeat(${weeks.length}, 1fr)` }, children: weeks.map((week, wi) => week.map((day, di) => (_jsx("div", { className: `heatmap-cell level-${getIntensityLevel(day.cost, maxCost)}`, title: `${day.date}: $${day.cost.toFixed(2)}`, style: { gridColumn: wi + 1, gridRow: di + 1 } }, `${wi}-${di}`)))) })] }), _jsxs("div", { className: "heatmap-legend", children: [_jsx("span", { children: t("heatmap.less") }), _jsx("div", { className: "heatmap-cell level-0" }), _jsx("div", { className: "heatmap-cell level-1" }), _jsx("div", { className: "heatmap-cell level-2" }), _jsx("div", { className: "heatmap-cell level-3" }), _jsx("div", { className: "heatmap-cell level-4" }), _jsx("span", { children: t("heatmap.more") })] })] }) }));
}
export default Heatmap;
//# sourceMappingURL=Heatmap.js.map