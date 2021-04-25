export function fib(n: number): number {
  // f(0) = 0
  let n1 = 0;
  // f(1) = 1
  let n2 = 1;
  let res = 0;
  for (let i = 0; i < n; i++) {
    // 问题要求的取模
    res = (n1 + n2) % 1000000007;
    n1 = n2;
    n2 = res;
  }
  return n1;
}

export function fib2(n: number): number {
  if (n == 0) return 0;
  if (n == 1) return 1;
  return fib2(n - 1) + fib2(n - 2);
}
