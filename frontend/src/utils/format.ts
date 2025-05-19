export const formatNumber = (num: number): string => {
  if (num === 0) return '0';
  if (Math.abs(num) < 0.01) return num.toExponential(2);
  if (Math.abs(num) < 1) return num.toFixed(4);
  if (Math.abs(num) < 100) return num.toFixed(2);
  return num.toLocaleString(undefined, {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}; 