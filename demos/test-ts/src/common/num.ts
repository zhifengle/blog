/**
 * @description convert number to percentage
 */
export function toPercent(num: number) {
  return num.toLocaleString("en-US", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
    style: "percent",
  });
}

// another way
export function toPercent2(num: number) {
  return (num / 100).toFixed(2) + "%";
}
