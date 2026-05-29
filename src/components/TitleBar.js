import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Square, X } from "lucide-react";
import BurnrLogo from "./BurnrLogo";
function TitleBar() {
    const appWindow = getCurrentWindow();
    const handleDragStart = (e) => {
        if (e.target.closest(".title-bar-controls"))
            return;
        e.preventDefault();
        appWindow.startDragging();
    };
    return (_jsxs("div", { className: "title-bar", onMouseDown: handleDragStart, children: [_jsx("div", { className: "title-bar-brand", children: _jsx(BurnrLogo, { size: 16 }) }), _jsxs("div", { className: "title-bar-controls", children: [_jsx("button", { className: "title-bar-btn", onClick: () => appWindow.minimize(), children: _jsx(Minus, { size: 14 }) }), _jsx("button", { className: "title-bar-btn", onClick: () => appWindow.toggleMaximize(), children: _jsx(Square, { size: 11 }) }), _jsx("button", { className: "title-bar-btn title-bar-close", onClick: () => appWindow.close(), children: _jsx(X, { size: 14 }) })] })] }));
}
export default TitleBar;
//# sourceMappingURL=TitleBar.js.map