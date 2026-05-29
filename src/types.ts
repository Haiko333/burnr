import { ReactNode } from "react";

export type Tool = "all" | "claude-code" | "codex" | "gemini" | "cursor" | "windsurf";

export interface ModelStats {
  model: string;
  inputTokens: number;
  outputTokens: number;
  cacheCreationTokens: number;
  cacheReadTokens: number;
  costUsd: number;
  entryCount: number;
}

export interface ProjectStats {
  project: string;
  totalInputTokens: number;
  totalOutputTokens: number;
  totalCacheCreationTokens: number;
  totalCacheReadTokens: number;
  totalCostUsd: number;
  costIsEstimated: boolean;
  sessionCount: number;
  entryCount: number;
  modelsUsed: ModelStats[];
}

export interface DailyUsage {
  date: string;
  inputTokens: number;
  outputTokens: number;
  cacheCreationTokens: number;
  cacheReadTokens: number;
  costUsd: number;
}

export interface GlobalStats {
  totalInputTokens: number;
  totalOutputTokens: number;
  totalCacheCreationTokens: number;
  totalCacheReadTokens: number;
  totalCostUsd: number;
  costIsEstimated: boolean;
  totalEntries: number;
  totalSessions: number;
  projects: ProjectStats[];
  dailyUsage: DailyUsage[];
}

export interface ToolAvailability {
  tool: string;
  available: boolean;
}

export interface ToolConfig {
  id: Tool;
  labelKey: string;
  icon: ReactNode;
}

export interface ToolLimits {
  tool: string;
  limitType: string;
  currentUsage: number;
  limitLabel: string;
  resetTime: string | null;
  requestsUsed: number | null;
  requestsTotal: number | null;
}
