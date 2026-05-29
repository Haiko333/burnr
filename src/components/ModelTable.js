import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useTranslation } from "react-i18next";
import { formatTokens, formatCost } from "../utils/format";
function ModelTable({ models, costIsEstimated }) {
    const { t } = useTranslation();
    const filtered = models.filter((m) => {
        if (m.model.includes("<synthetic>"))
            return false;
        const total = m.inputTokens + m.outputTokens + m.cacheReadTokens + m.cacheCreationTokens;
        return total > 0 || m.costUsd > 0;
    });
    const sorted = [...filtered].sort((a, b) => b.costUsd - a.costUsd);
    const totalCost = sorted.reduce((sum, m) => sum + m.costUsd, 0);
    return (_jsx("div", { className: "model-table-container", children: _jsx("div", { className: "table-wrapper", children: _jsxs("table", { className: "model-table", children: [_jsx("thead", { children: _jsxs("tr", { children: [_jsx("th", { children: t("table.model") }), _jsx("th", { children: t("table.input") }), _jsx("th", { children: t("table.output") }), _jsx("th", { children: t("table.cacheRead") }), _jsx("th", { children: t("table.cacheWrite") }), _jsx("th", { children: t("table.total") }), _jsx("th", { children: t("table.cost") }), _jsx("th", { children: t("table.pct") })] }) }), _jsx("tbody", { children: sorted.map((m) => {
                            const total = m.inputTokens + m.outputTokens + m.cacheReadTokens + m.cacheCreationTokens;
                            const pct = totalCost > 0 ? (m.costUsd / totalCost) * 100 : 0;
                            return (_jsxs("tr", { children: [_jsx("td", { className: "model-name", children: m.model }), _jsx("td", { children: formatTokens(m.inputTokens) }), _jsx("td", { children: formatTokens(m.outputTokens) }), _jsx("td", { children: formatTokens(m.cacheReadTokens) }), _jsx("td", { children: formatTokens(m.cacheCreationTokens) }), _jsx("td", { className: "total-col", children: formatTokens(total) }), _jsx("td", { className: "cost-col", children: formatCost(m.costUsd, costIsEstimated) }), _jsxs("td", { className: "pct-col", children: [pct.toFixed(1), "%"] })] }, m.model));
                        }) })] }) }) }));
}
export default ModelTable;
//# sourceMappingURL=ModelTable.js.map