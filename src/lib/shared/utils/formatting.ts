export function formatDuration(startTime: string, endTime?: string) {
  if (!startTime) return '';

  const start = new Date(startTime);
  const end = endTime ? new Date(endTime) : new Date();
  const durationMs = end.getTime() - start.getTime();

  if (durationMs < 1000) return '<1s';
  if (durationMs < 60000) return `${Math.round(durationMs / 1000)}s`;
  if (durationMs < 3600000) return `${Math.round(durationMs / 60000)}m`;
  return `${Math.round(durationMs / 3600000)}h`;
}

export function formatTimestamp(timestamp: string) {
  if (!timestamp) return 'Unknown';
  try {
    return new Date(timestamp).toLocaleString();
  } catch {
    return 'Invalid date';
  }
}