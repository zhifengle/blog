function removeKdigits(num: string, k: number): string {
  let stack: string[] = [];

  const remain = num.length - k;
  for (let i = 0; i < num.length; i++) {
    const c = num[i];
    while (k && stack.length && stack[stack.length - 1] > c) {
      stack.pop();
      k -= 1;
    }
    stack.push(c);
  }
  return stack.slice(0, remain).join('').replace(/^0*/, '') || '0';
}

function removeKdigits2(num: string, k: number): string {
  const remain = num.length - k;
  const result = Array(remain).fill('');
  let idx = 0;
  for (let i = 0; i < num.length; i++) {
    const c = num[i];
    while (idx > 0 && remain - idx < num.length - i && result[idx - 1] > c) {
      idx--;
    }
    if (idx < remain) {
      result[idx++] = c;
    }
  }
  return result.join('').replace(/^0*/, '') || '0';
}

console.log(removeKdigits2('1432219', 3));
