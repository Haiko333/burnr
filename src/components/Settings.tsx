import { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { X, Download, HelpCircle, Eye, EyeOff } from "lucide-react";
import { GlobalStats } from "../types";
import { Theme } from "../hooks/useTheme";

interface SessionTokenInfo {
  tool: string;
  hasToken: boolean;
  source: "manual" | "none";
  browser: string | null;
  maskedToken: string | null;
  maskedOrg: string | null;
}

interface SettingsProps {
  isOpen: boolean;
  onClose: () => void;
  theme: Theme;
  onThemeChange: (theme: Theme) => void;
  stats: GlobalStats | null;
}

const FORMULA_PREFIXES = ["=", "+", "-", "@", "\t", "\r"];

function getErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message;
  return String(error);
}

function escapeCsvValue(value: string | number): string {
  const raw = String(value);
  const safeValue = FORMULA_PREFIXES.some((prefix) => raw.startsWith(prefix)) ? `'${raw}` : raw;
  return `"${safeValue.replace(/"/g, '""')}"`;
}

function exportAsJson(stats: GlobalStats) {
  const blob = new Blob([JSON.stringify(stats, null, 2)], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `burnr-export-${new Date().toISOString().slice(0, 10)}.json`;
  a.click();
  URL.revokeObjectURL(url);
}

function exportAsCsv(stats: GlobalStats) {
  const header = "project,model,input_tokens,output_tokens,cache_read,cache_write,cost_usd";
  const rows = stats.projects.flatMap((project) =>
    project.modelsUsed.map((model) =>
      [
        project.project,
        model.model,
        model.inputTokens,
        model.outputTokens,
        model.cacheReadTokens,
        model.cacheCreationTokens,
        model.costUsd.toFixed(6),
      ].map(escapeCsvValue).join(",")
    )
  );

  const blob = new Blob([[header, ...rows].join("\n")], { type: "text/csv" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `burnr-export-${new Date().toISOString().slice(0, 10)}.csv`;
  a.click();
  URL.revokeObjectURL(url);
}

function Settings({ isOpen, onClose, theme, onThemeChange, stats }: SettingsProps) {
  const { t, i18n } = useTranslation();
  const [tokens, setTokens] = useState<SessionTokenInfo[]>([]);
  const [tokenInputs, setTokenInputs] = useState<Record<string, string>>({});
  const [helpForTool, setHelpForTool] = useState<string | null>(null);
  const [visibleTokens, setVisibleTokens] = useState<Record<string, boolean>>({});
  const [tokenMessage, setTokenMessage] = useState<{ type: "success" | "error"; text: string } | null>(null);

  useEffect(() => {
    if (isOpen) {
      setTokenMessage(null);
      invoke<SessionTokenInfo[]>("get_session_tokens")
        .then(setTokens)
        .catch((err) => {
          setTokenMessage({ type: "error", text: getErrorMessage(err) });
        });
    }
  }, [isOpen]);

  if (!isOpen) return null;

  const handleLanguageChange = (lang: string) => {
    i18n.changeLanguage(lang);
    localStorage.setItem("burnr-language", lang);
  };

  const handleSaveToken = async (tool: string) => {
    const val = tokenInputs[tool]?.trim();
    if (!val) return;
    const orgVal = tokenInputs[`${tool}_org`]?.trim() || undefined;
    try {
      setTokenMessage(null);
      await invoke("set_session_token", { tool, token: val, orgId: orgVal });
      const refreshedTokens = await invoke<SessionTokenInfo[]>("get_session_tokens");
      setTokens(refreshedTokens);
      setTokenInputs((prev) => ({ ...prev, [tool]: "", [`${tool}_org`]: "" }));
      setTokenMessage({ type: "success", text: t("settings.tokenSaved") });
    } catch (err) {
      setTokenMessage({ type: "error", text: getErrorMessage(err) });
    }
  };

  return (
    <div className="settings-overlay" onClick={onClose}>
      <div className="settings-panel" onClick={(e) => e.stopPropagation()}>
        <div className="settings-header">
          <h2>{t("settings.title")}</h2>
          <button className="settings-close" onClick={onClose}>
            <X size={18} />
          </button>
        </div>

        <div className="settings-body">
          <div className="settings-section">
            <h3>{t("settings.language")}</h3>
            <div className="settings-toggle-group">
              <button
                className={`settings-toggle ${i18n.language === "en" ? "active" : ""}`}
                onClick={() => handleLanguageChange("en")}
              >
                English
              </button>
              <button
                className={`settings-toggle ${i18n.language === "fr" ? "active" : ""}`}
                onClick={() => handleLanguageChange("fr")}
              >
                Francais
              </button>
            </div>
          </div>

          <div className="settings-section">
            <h3>{t("settings.theme")}</h3>
            <div className="settings-toggle-group">
              <button
                className={`settings-toggle ${theme === "dark" ? "active" : ""}`}
                onClick={() => onThemeChange("dark")}
              >
                {t("settings.dark")}
              </button>
              <button
                className={`settings-toggle ${theme === "light" ? "active" : ""}`}
                onClick={() => onThemeChange("light")}
              >
                {t("settings.light")}
              </button>
            </div>
          </div>

          <div className="settings-section">
            <h3>{t("settings.sessionTokens")}</h3>
            {tokenMessage && (
              <div className={`settings-message ${tokenMessage.type}`}>
                {tokenMessage.text}
              </div>
            )}
            <div className="settings-tokens">
              {tokens.map((tok) => (
                <div key={tok.tool} className="settings-token-row">
                  <div className="settings-token-info">
                    <div className="settings-token-name">
                      <span className="settings-token-tool">{tok.tool}</span>
                      <button
                        className="settings-help-btn"
                        onClick={() => setHelpForTool(tok.tool)}
                        title={t("settings.howToGetToken")}
                      >
                        <HelpCircle size={13} />
                      </button>
                    </div>
                    <span className={`settings-token-status ${tok.hasToken ? "connected" : ""}`}>
                      {tok.source === "manual"
                        ? t("settings.manual")
                        : t("settings.notConfigured")}
                    </span>
                  </div>
                  {tok.hasToken && tok.maskedToken && (
                    <div className="settings-token-saved">
                      <code className="settings-token-masked">
                        {visibleTokens[tok.tool] ? tok.maskedToken : "••••••••••••"}
                      </code>
                      {tok.maskedOrg && (
                        <code className="settings-token-masked">
                          {visibleTokens[tok.tool] ? tok.maskedOrg : "••••••••"}
                        </code>
                      )}
                      <button
                        className="settings-token-toggle"
                        onClick={() => setVisibleTokens((prev) => ({ ...prev, [tok.tool]: !prev[tok.tool] }))}
                      >
                        {visibleTokens[tok.tool] ? <EyeOff size={13} /> : <Eye size={13} />}
                      </button>
                    </div>
                  )}
                  <div className="settings-token-input">
                    <input
                      type="password"
                      placeholder={tok.hasToken ? t("settings.overrideToken") : t("settings.tokenPlaceholder")}
                      value={tokenInputs[tok.tool] || ""}
                      onChange={(e) =>
                        setTokenInputs((prev) => ({ ...prev, [tok.tool]: e.target.value }))
                      }
                    />
                    {tok.tool === "claude" && (
                      <input
                        type="text"
                        placeholder={t("settings.orgIdPlaceholder")}
                        value={tokenInputs[`${tok.tool}_org`] || ""}
                        onChange={(e) =>
                          setTokenInputs((prev) => ({ ...prev, [`${tok.tool}_org`]: e.target.value }))
                        }
                      />
                    )}
                    <button
                      className="settings-token-save"
                      onClick={() => handleSaveToken(tok.tool)}
                      disabled={!tokenInputs[tok.tool]}
                    >
                      {t("settings.save")}
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>

          <div className="settings-section">
            <h3>{t("settings.export")}</h3>
            <div className="settings-actions">
              <button
                className="settings-btn"
                onClick={() => stats && exportAsJson(stats)}
                disabled={!stats}
              >
                <Download size={14} />
                {t("settings.exportJson")}
              </button>
              <button
                className="settings-btn"
                onClick={() => stats && exportAsCsv(stats)}
                disabled={!stats}
              >
                <Download size={14} />
                {t("settings.exportCsv")}
              </button>
            </div>
          </div>

        </div>

        {helpForTool && (
          <div className="token-help-overlay" onClick={() => setHelpForTool(null)}>
            <div className="token-help-modal" onClick={(e) => e.stopPropagation()}>
              <div className="token-help-header">
                <h3>{t(`settings.helpTitle_${helpForTool}`)}</h3>
                <button className="settings-close" onClick={() => setHelpForTool(null)}>
                  <X size={16} />
                </button>
              </div>
              <div className="token-help-body">
                <div className="token-help-step">
                  <span className="token-help-number">1</span>
                  <p>{t(`settings.help_${helpForTool}_step1`)}</p>
                </div>
                <div className="token-help-step">
                  <span className="token-help-number">2</span>
                  <p>{t(`settings.help_${helpForTool}_step2`)}</p>
                </div>
                <div className="token-help-step">
                  <span className="token-help-number">3</span>
                  <p>{t(`settings.help_${helpForTool}_step3`)}</p>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default Settings;
