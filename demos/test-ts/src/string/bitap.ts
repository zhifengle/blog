interface IAlphabet {
  [propName: string]: number;
}

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
