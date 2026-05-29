import { jsx as _jsx } from "react/jsx-runtime";
const COLORS = [
    "#c4b5fd", "#a78bfa", "#8b5cf6",
    "#8b5cf6", "#7c3aed", "#6d28d9",
    "#6d28d9", "#5b21b6", "#4c1d95",
];
function BurnrLogo({ size = 20 }) {
    const cell = (size - 4) / 3; // 2px gap × 2 = 4px total gap
    const gap = 2;
    const r = 2;
    return (_jsx("svg", { width: size, height: size, viewBox: `0 0 ${size} ${size}`, fill: "none", children: COLORS.map((color, i) => {
            const col = i % 3;
            const row = Math.floor(i / 3);
            const x = col * (cell + gap);
            const y = row * (cell + gap);
            return (_jsx("rect", { x: x, y: y, width: cell, height: cell, rx: r, ry: r, fill: color }, i));
        }) }));
}
export default BurnrLogo;
//# sourceMappingURL=BurnrLogo.js.map