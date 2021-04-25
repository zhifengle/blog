export function addBinary(a: string, b: string): string {
  const lenA = a.length;
  const lenB = b.length;
  let res = '';
  let i = lenA - 1;
  let j = lenB - 1;
  let carry = 0;
  while (i >= 0 || j >= 0 || carry > 0) {
    let num1 = 0;
    let num2 = 0;
    let ans = 0;
    if (carry > 0) {
      ans += carry;
      carry = 0;
    }
    if (i >= 0) {
      num1 = Number(a[i]);
    }
    if (j >= 0) {
      num2 = Number(b[j]);
    }
    ans = num1 + num2 + ans;

    res = (ans % 2) + res + '';
    if (ans >= 2) {
      carry = 1;
    }
    i--;
    j--;
  }
  return res;
}
