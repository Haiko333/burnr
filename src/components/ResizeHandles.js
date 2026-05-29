import { jsx as _jsx, Fragment as _Fragment } from "react/jsx-runtime";
import { getCurrentWindow } from "@tauri-apps/api/window";
const DIRECTIONS = [
    { direction: "North", className: "resize-handle resize-handle-n" },
    { direction: "South", className: "resize-handle resize-handle-s" },
    { direction: "East", className: "resize-handle resize-handle-e" },
    { direction: "West", className: "resize-handle resize-handle-w" },
    { direction: "NorthEast", className: "resize-handle resize-handle-ne" },
    { direction: "NorthWest", className: "resize-handle resize-handle-nw" },
    { direction: "SouthEast", className: "resize-handle resize-handle-se" },
    { direction: "SouthWest", className: "resize-handle resize-handle-sw" },
];
function ResizeHandles() {
    const appWindow = getCurrentWindow();
    const handleMouseDown = (direction) => (e) => {
        e.preventDefault();
        appWindow.startResizeDragging(direction);
    };
    return (_jsx(_Fragment, { children: DIRECTIONS.map(({ direction, className }) => (_jsx("div", { className: className, onMouseDown: handleMouseDown(direction) }, direction))) }));
}
export default ResizeHandles;
//# sourceMappingURL=ResizeHandles.js.map