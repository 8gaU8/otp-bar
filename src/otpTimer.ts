/**
 * Calculate the remaining time in seconds for the current OTP period
 * OTP typically refreshes every 30 seconds based on Unix time
 * @returns Remaining time in seconds (0-29)
 */
export function getOTPRemainingTime(): number {
  const now = Math.floor(Date.now() / 1000); // Current Unix time in seconds
  const period = 30; // OTP period in seconds
  const timeInPeriod = now % period;
  const remainingTime = period - timeInPeriod;
  return remainingTime;
}

/**
 * Check if the OTP is in the warning period (last 10 seconds)
 * @returns true if remaining time is <= 10 seconds
 */
export function isOTPInWarningPeriod(): boolean {
  return getOTPRemainingTime() <= 10;
}

/**
 * Format remaining time for display
 * @returns Formatted string like "15s" or "5s"
 */
export function formatRemainingTime(): string {
  const remaining = getOTPRemainingTime();
  return `${remaining}s`;
}
