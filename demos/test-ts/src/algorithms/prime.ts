// [打印质数的各种算法](https://coolshell.cn/articles/3738.html)

export function isPrime(n: number): boolean {
  for (let i = 2; i * i < n + 1; i++) {
    if (!(n % i)) return false
  }
  return true
}
