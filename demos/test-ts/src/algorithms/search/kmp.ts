// https://gist.github.com/blasten/d42bd0d814b7df1addea
// @TODO
function longestPrefix(str: string) {
  // str[0..i]
  const table = new Array(str.length);
  let maxPrefix = 0;
  table[0] = 0;
  for (let i = 1; i < str.length; i++) {
    while (maxPrefix > 0 && str.charAt(i) !== str.charAt(maxPrefix)) {
      maxPrefix = table[maxPrefix - 1];
    }
  }
}
export function kmp(text: string, pattern: string) {
  let i = 0;
  let j = 0;
  const next = getNext(pattern);
  while (i < text.length && j < pattern.length) {
    if (j === -1 || text[i] === pattern[j]) {
      i++;
      j++;
    } else {
      // 移动的 j。一般的教程说的是移动 i
      j = next[j];
    }
  }
  if (j == pattern.length) {
    return i - j;
  } else {
    return -1;
  }
}
// PMT ----> 右移 --->  next
function getNext(pattern: string) {
  const next = new Array(pattern.length);
  // 没有特殊含义 编程方便
  next[0] = -1;
  let i = 0;
  let j = -1;
  // while (i < pattern.length) 原答主的答案越界了
  // 先再赋值的
  while (i < pattern.length - 1) {
    if (j == -1 || pattern[i] === pattern[j]) {
      ++i;
      ++j;
      next[i] = j;
    } else {
      j = next[j];
    }
  }

  return next;
}

// -1 0 0 1 2 3 4 0
console.log(kmp('hello', 'll'));
