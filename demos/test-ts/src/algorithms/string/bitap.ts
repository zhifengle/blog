// https://stackoverflow.com/questions/11317973/string-similarity-how-exactly-does-bitap-work
// https://github.com/tallesl/node-bitap/blob/master/index.js
// https://github.com/heyimalex/bitap

type IAlphabet = Record<string, number>;

interface IFuseOptions {
  isCaseSensitive?: boolean;
  distance?: number;
  findAllMatches?: boolean;
  includeMatches?: boolean;
  includeScore?: boolean;
  location?: number;
  minMatchCharLength?: number;
  shouldSort?: boolean;
  threshold?: number;
  useExtendedSearch?: boolean;
}

// ababc
// m['a'] = 20; '10100'; 这个算法是标记存在的位置为 1;
// stackoverflow 上面是标记 0
function patternAlphabet(pattern: string): IAlphabet {
  let mask: IAlphabet = {};
  let len = pattern.length;

  for (let i = 0; i < len; i += 1) {
    mask[pattern.charAt(i)] = 0;
  }

  for (let i = 0; i < len; i += 1) {
    mask[pattern.charAt(i)] |= 1 << (len - i - 1);
  }

  return mask;
}

/**
 * bitap 算法
 * @param text 搜索字符串
 * @param pattern 目标字符串
 * @param param2 设置选项
 */
export function bitapSearch(
  text: string,
  pattern: string,
  {
    location = 0,
    distance = 100,
    threshold = 0.6,
    findAllMatches = false,
    minMatchCharLength = 1,
    includeMatches = false,
  }: IFuseOptions = {}
) {
  const patternLen = pattern.length;
  const textLen = text.length;

  const expectedLocation = Math.max(0, Math.min(location, textLen));
  let currentThreshold = threshold;
  // bestLocation 省略
  let bestLocation = -1;

  const matchMask = new Array(textLen).fill(0);

  let lastBitArr = [];
  let finalScore = 1;
  let binMax = patternLen + textLen;

  const mask = 1 << (patternLen <= 31 ? patternLen - 1 : 30);
}

// bitap search. 精确搜索
// 一个反直觉的地方是 匹配时标记 0， 1标记为不匹配
// 时间 O(mn)
// ababc  a: 01011  ---->   11010 ;  b: 10101 ---> 10101

function bitapBitWiseSearch(text: string, pattern: string): number {
  const m = pattern.length;
  let R = ~1;
  if (m === 0) {
    return -1;
  }
  if (m > 31) {
    console.log('Pattern too long');
    return -1;
  }
  // 初始化位掩码
  let patternMask: number[] = Array.from(Array(127), (v, k) => ~0);

  // len loop
  for (let i = 0; i < m; ++i) {
    patternMask[pattern[i].charCodeAt(0)] &= ~(1 << i);
  }
  for (let i = 0; i < text.length; ++i) {
    // 更新位数组
    R |= patternMask[text[i].charCodeAt(0)];
    R <<= 1;
    if ((R & (1 << m)) === 0) return i - m + 1;
  }

  return -1;
}

console.log(bitapBitWiseSearch('hello', 'o'));
// console.log(patternAlphabet('ababc'));
