import type { InputNumberProps } from "antd";

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

/**
 * 
 * @param value - The number to format
 * @returns Formatted string with commas
 */
export const inputFormatter: InputNumberProps<number>['formatter'] = (value) => {
  const [start, end] = `${value}`.split('.') || [];
  const v = `${start}`.replace(/\B(?=(\d{3})+(?!\d))/g, ',');
  return `${end ? `${v}.${end}` : `${v}`}`;
};