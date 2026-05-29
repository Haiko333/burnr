import { useTranslation } from "react-i18next";
import { formatTokens, formatCost } from "../utils/format";
import { ModelStats } from "../types";

interface ModelTableProps {
  models: ModelStats[];
  costIsEstimated: boolean;
}

function ModelTable({ models, costIsEstimated }: ModelTableProps) {
  const { t } = useTranslation();
  const filtered = models.filter((m) => {
    if (m.model.includes("<synthetic>")) return false;
    const total = m.inputTokens + m.outputTokens + m.cacheReadTokens + m.cacheCreationTokens;
    return total > 0 || m.costUsd > 0;
  });
  const sorted = [...filtered].sort((a, b) => b.costUsd - a.costUsd);
  const totalCost = sorted.reduce((sum, m) => sum + m.costUsd, 0);

  return (
    <div className="model-table-container">
      <div className="table-wrapper">
        <table className="model-table">
          <thead>
            <tr>
              <th>{t("table.model")}</th>
              <th>{t("table.input")}</th>
              <th>{t("table.output")}</th>
              <th>{t("table.cacheRead")}</th>
              <th>{t("table.cacheWrite")}</th>
              <th>{t("table.total")}</th>
              <th>{t("table.cost")}</th>
              <th>{t("table.pct")}</th>
            </tr>
          </thead>
          <tbody>
            {sorted.map((m) => {
              const total =
                m.inputTokens + m.outputTokens + m.cacheReadTokens + m.cacheCreationTokens;
              const pct = totalCost > 0 ? (m.costUsd / totalCost) * 100 : 0;
              return (
                <tr key={m.model}>
                  <td className="model-name">{m.model}</td>
                  <td>{formatTokens(m.inputTokens)}</td>
                  <td>{formatTokens(m.outputTokens)}</td>
                  <td>{formatTokens(m.cacheReadTokens)}</td>
                  <td>{formatTokens(m.cacheCreationTokens)}</td>
                  <td className="total-col">{formatTokens(total)}</td>
                  <td className="cost-col">{formatCost(m.costUsd, costIsEstimated)}</td>
                  <td className="pct-col">{pct.toFixed(1)}%</td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default ModelTable;
