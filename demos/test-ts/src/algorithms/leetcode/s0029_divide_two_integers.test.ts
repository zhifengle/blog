// js 不会出现 i32 溢出的情况
function divide(dividend: number, divisor: number): number {
  if (divisor === 1) return dividend;
  if (dividend === divisor) return 1;
  const INT_MAX = 2147483647;
  const INT_MIN = -INT_MAX - 1;
  if (dividend == INT_MIN && divisor == -1) {
    return INT_MAX;
  }
  if (dividend == 0 || divisor == INT_MIN) {
    return 0;
  }
  const isNeg = dividend > 0 !== divisor > 0;
  let dvd = Math.abs(dividend);
  let dvs = Math.abs(divisor);

  let res = 0;
  let sub = dvs;
  let count = 1;
  while (dvd >= dvs) {
    count = 1;
    sub = dvs;
    while (sub <= dvd >> 1) {
      sub <<= 1;
      count <<= 1;
    }
    res += count;
    dvd -= sub;
  }

  if (res > INT_MAX || res < INT_MIN) {
    return INT_MAX;
  }

  return isNeg ? -res : res;
}

test('s0029', () => {
  expect(divide(10, 3)).toBe(3);
  expect(divide(7, -3)).toBe(-2);
  expect(divide(-2147483648, -1)).toBe(2147483647);
  expect(divide(-2147483648, -2147483648)).toBe(1);
  expect(divide(2147483647, 2)).toBe(1073741823);
});
