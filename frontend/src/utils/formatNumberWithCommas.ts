/**
 * Formats numbers greater than 999 with commas as thousand separators
 * @param num - The number to format
 * @returns Formatted string with commas for numbers > 999, original number as string otherwise
 */
export function formatNumberWithCommas(num: number): string {
  if (num < -999 || num > 999) {
    return num.toLocaleString('en-US');
  }
  return num.toString();
}