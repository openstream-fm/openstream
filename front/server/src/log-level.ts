// moved to separate file to avoid circular dependency issue

// fix for bug in vite about enums
export const LogLevel = {
  SILENT: 0 as const,
  ERROR: 1 as const,
  WARN: 2 as const,
  INFO: 3 as const,
  DEBUG: 4 as const,
}

export type LogLevel = 0 | 1 | 2 | 3 | 4
