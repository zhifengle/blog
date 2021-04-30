// TODO: [马拉车算法](https://blog.csdn.net/qq_43152052/article/details/100784978)
// 规律1. 新的补充字符串回文最大半径减一就是最长回文.  这里缺起始位置索引
// 规律2. 最长回文的起始位置是中间位置减去半径除以2
export function longestPalindrome2(s: string): string {
  if (s.length <= 1) return s;
  let t = '$#';
  for (let i = 0; i < s.length; i++) {
    t += s[i];
    t += '#';
  }
  const p = [];

  let mx = 0; // right idx
  let id = 0; // center idx
  let resLen = 0; // s 中的 半径
  let resCenter = 0; //  s 中的最大回文的中间
}

export function longestPalindrome(s: string): string {
  if (s.length <= 1) return s;
  const find = (s: string, left: number, right: number) => {
    while (left >= 0 && right <= s.length - 1 && s[left] === s[right]) {
      left--;
      right++;
    }
    return s.slice(left + 1, right);
  };
  let str = '';
  let longest = '';
  for (let i = 0; i < s.length - 1; i++) {
    str = find(s, i, i);
    if (str.length > longest.length) {
      longest = str;
    }
    str = find(s, i, i + 1);
    if (str.length > longest.length) {
      longest = str;
    }
  }
  return longest;
}
