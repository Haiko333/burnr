import { useState, useEffect, useCallback } from "react";
export function useTheme() {
    const [theme, setThemeState] = useState(() => {
        return localStorage.getItem("burnr-theme") || "dark";
    });
    useEffect(() => {
        document.documentElement.setAttribute("data-theme", theme);
        localStorage.setItem("burnr-theme", theme);
    }, [theme]);
    const setTheme = useCallback((t) => {
        setThemeState(t);
    }, []);
    return { theme, setTheme };
}
//# sourceMappingURL=useTheme.js.map