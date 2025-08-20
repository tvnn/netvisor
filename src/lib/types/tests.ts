export interface TestResult {
  success: boolean;
  message: string;
  duration_ms: number;
  executed_at: string;
  criticality?: string;
  details?: any;
}