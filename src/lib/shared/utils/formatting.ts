export const uuidv4Sentinel: string = '00000000-0000-0000-0000-000000000000';

export const utcTimeZoneSentinel: string = "1970-01-01T00:00:00Z";

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

export function formatTimestamp(timestamp: string): string {
  try {
    const date = new Date(timestamp);
    return date.toLocaleString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      hour12: false
    });
  } catch (error) {
    return timestamp; // Fallback to raw string if parsing fails
  }
}
  
  // Truncate ID for display (show first 8 characters + ellipsis if longer than 12)
export function formatId(id: string): string {
  if (id.length <= 12) {
    return id;
  }
  return `${id.substring(0, 8)}...`;
}