import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { X, Download, Upload, HelpCircle } from "lucide-react";
function exportAsJson(stats) {
    const blob = new Blob([JSON.stringify(stats, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `burnr-export-${new Date().toISOString().slice(0, 10)}.json`;
    a.click();
    URL.revokeObjectURL(url);
}
function exportAsCsv(stats) {
    const rows = ["project,model,input_tokens,output_tokens,cache_read,cache_write,cost_usd"];
    for (const project of stats.projects) {
        for (const model of project.modelsUsed) {
            rows.push([
                project.project,
                model.model,
                model.inputTokens,
                model.outputTokens,
                model.cacheReadTokens,
                model.cacheCreationTokens,
                model.costUsd.toFixed(6),
            ].join(","));
        }
    }
    const blob = new Blob([rows.join("\n")], { type: "text/csv" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `burnr-export-${new Date().toISOString().slice(0, 10)}.csv`;
    a.click();
    URL.revokeObjectURL(url);
}
function Settings({ isOpen, onClose, theme, onThemeChange, stats }) {
    const { t, i18n } = useTranslation();
    const [importStrategy, setImportStrategy] = useState("merge");
    const [tokens, setTokens] = useState([]);
    const [tokenInputs, setTokenInputs] = useState({});
    const [helpForTool, setHelpForTool] = useState(null);
    useEffect(() => {
        if (isOpen) {
            invoke("get_session_tokens").then(setTokens).catch(() => { });
        }
    }, [isOpen]);
    if (!isOpen)
        return null;
    const handleLanguageChange = (lang) => {
        i18n.changeLanguage(lang);
        localStorage.setItem("burnr-language", lang);
    };
    const handleImport = () => {
        const input = document.createElement("input");
        input.type = "file";
        input.accept = ".json,.csv";
        input.onchange = (e) => {
            const file = e.target.files?.[0];
            if (!file)
                return;
            const reader = new FileReader();
            reader.onload = () => {
                void importStrategy;
                void reader.result;
            };
            reader.readAsText(file);
        };
        input.click();
    };
    const handleSaveToken = async (tool) => {
        const val = tokenInputs[tool];
        if (!val)
            return;
        const orgVal = tokenInputs[`${tool}_org`] || undefined;
        try {
            await invoke("set_session_token", { tool, token: val, orgId: orgVal });
            setTokens((prev) => prev.map((t) => (t.tool === tool ? { ...t, hasToken: true, source: "manual" } : t)));
            setTokenInputs((prev) => ({ ...prev, [tool]: "", [`${tool}_org`]: "" }));
        }
        catch {
            // handle error silently
        }
    };
    return (_jsx("div", { className: "settings-overlay", onClick: onClose, children: _jsxs("div", { className: "settings-panel", onClick: (e) => e.stopPropagation(), children: [_jsxs("div", { className: "settings-header", children: [_jsx("h2", { children: t("settings.title") }), _jsx("button", { className: "settings-close", onClick: onClose, children: _jsx(X, { size: 18 }) })] }), _jsxs("div", { className: "settings-body", children: [_jsxs("div", { className: "settings-section", children: [_jsx("h3", { children: t("settings.language") }), _jsxs("div", { className: "settings-toggle-group", children: [_jsx("button", { className: `settings-toggle ${i18n.language === "en" ? "active" : ""}`, onClick: () => handleLanguageChange("en"), children: "English" }), _jsx("button", { className: `settings-toggle ${i18n.language === "fr" ? "active" : ""}`, onClick: () => handleLanguageChange("fr"), children: "Francais" })] })] }), _jsxs("div", { className: "settings-section", children: [_jsx("h3", { children: t("settings.theme") }), _jsxs("div", { className: "settings-toggle-group", children: [_jsx("button", { className: `settings-toggle ${theme === "dark" ? "active" : ""}`, onClick: () => onThemeChange("dark"), children: t("settings.dark") }), _jsx("button", { className: `settings-toggle ${theme === "light" ? "active" : ""}`, onClick: () => onThemeChange("light"), children: t("settings.light") })] })] }), _jsxs("div", { className: "settings-section", children: [_jsx("h3", { children: t("settings.sessionTokens") }), _jsx("div", { className: "settings-tokens", children: tokens.map((tok) => (_jsxs("div", { className: "settings-token-row", children: [_jsxs("div", { className: "settings-token-info", children: [_jsxs("div", { className: "settings-token-name", children: [_jsx("span", { className: "settings-token-tool", children: tok.tool }), _jsx("button", { className: "settings-help-btn", onClick: () => setHelpForTool(tok.tool), title: t("settings.howToGetToken"), children: _jsx(HelpCircle, { size: 13 }) })] }), _jsx("span", { className: `settings-token-status ${tok.hasToken ? "connected" : ""}`, children: tok.source === "detected"
                                                            ? `${t("settings.detected")} (${tok.browser})`
                                                            : tok.source === "manual"
                                                                ? t("settings.manual")
                                                                : t("settings.notConfigured") })] }), _jsxs("div", { className: "settings-token-input", children: [_jsx("input", { type: "password", placeholder: tok.hasToken ? t("settings.overrideToken") : "Session key", value: tokenInputs[tok.tool] || "", onChange: (e) => setTokenInputs((prev) => ({ ...prev, [tok.tool]: e.target.value })) }), tok.tool === "claude" && (_jsx("input", { type: "text", placeholder: "Org ID", value: tokenInputs[`${tok.tool}_org`] || "", onChange: (e) => setTokenInputs((prev) => ({ ...prev, [`${tok.tool}_org`]: e.target.value })) })), _jsx("button", { className: "settings-token-save", onClick: () => handleSaveToken(tok.tool), disabled: !tokenInputs[tok.tool], children: t("settings.save") })] })] }, tok.tool))) })] }), _jsxs("div", { className: "settings-section", children: [_jsx("h3", { children: t("settings.export") }), _jsxs("div", { className: "settings-actions", children: [_jsxs("button", { className: "settings-btn", onClick: () => stats && exportAsJson(stats), disabled: !stats, children: [_jsx(Download, { size: 14 }), t("settings.exportJson")] }), _jsxs("button", { className: "settings-btn", onClick: () => stats && exportAsCsv(stats), disabled: !stats, children: [_jsx(Download, { size: 14 }), t("settings.exportCsv")] })] })] }), _jsxs("div", { className: "settings-section", children: [_jsx("h3", { children: t("settings.import") }), _jsxs("div", { className: "settings-import", children: [_jsxs("div", { className: "settings-toggle-group", children: [_jsx("button", { className: `settings-toggle ${importStrategy === "merge" ? "active" : ""}`, onClick: () => setImportStrategy("merge"), children: t("settings.merge") }), _jsx("button", { className: `settings-toggle ${importStrategy === "overwrite" ? "active" : ""}`, onClick: () => setImportStrategy("overwrite"), children: t("settings.overwrite") })] }), _jsxs("button", { className: "settings-btn", onClick: handleImport, children: [_jsx(Upload, { size: 14 }), t("settings.importFile")] })] })] })] }), helpForTool && (_jsx("div", { className: "token-help-overlay", onClick: () => setHelpForTool(null), children: _jsxs("div", { className: "token-help-modal", onClick: (e) => e.stopPropagation(), children: [_jsxs("div", { className: "token-help-header", children: [_jsx("h3", { children: t(`settings.helpTitle_${helpForTool}`) }), _jsx("button", { className: "settings-close", onClick: () => setHelpForTool(null), children: _jsx(X, { size: 16 }) })] }), _jsxs("div", { className: "token-help-body", children: [_jsxs("div", { className: "token-help-step", children: [_jsx("span", { className: "token-help-number", children: "1" }), _jsx("p", { children: t(`settings.help_${helpForTool}_step1`) })] }), _jsxs("div", { className: "token-help-step", children: [_jsx("span", { className: "token-help-number", children: "2" }), _jsx("p", { children: t(`settings.help_${helpForTool}_step2`) })] }), _jsxs("div", { className: "token-help-step", children: [_jsx("span", { className: "token-help-number", children: "3" }), _jsx("p", { children: t(`settings.help_${helpForTool}_step3`) })] })] })] }) }))] }) }));
}
export default Settings;
//# sourceMappingURL=Settings.js.map