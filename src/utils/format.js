export function formatCost(cost, isEstimated) {
    const formatted = `$${cost.toFixed(2)}`;
    return isEstimated ? `~ ${formatted}` : formatted;
}
export function formatTokens(n) {
    if (n >= 1000000)
        return `${(n / 1000000).toFixed(1)}M`;
    if (n >= 1000)
        return `${(n / 1000).toFixed(1)}K`;
    return n.toString();
}
//# sourceMappingURL=format.js.map