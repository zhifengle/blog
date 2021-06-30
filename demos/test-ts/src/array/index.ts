export function flatten<T>(a: T | T[]): T[] {
  return Array.isArray(a) ? [].concat.apply([], a.map(flatten)) : a;
}

// TODO type
export function deepCopy<T>(original: any): any {
  if (Array.isArray(original)) {
    const copy = [];
    for (const [idx, val] of original.entries()) {
      copy[idx] = deepCopy(val);
    }
    return copy;
  } else if (original && typeof original === 'object') {
    const copy: any = {};
    for (const [key, val] of original.entries()) {
      copy[key] = deepCopy(val);
    }
    return copy;
  }
  return original;
}

/**
 * 排序数组，根据原始和新的索引排
 * @param valArr 需要排序的数据
 * @param origin 原始索引
 * @param target 目标索引
 * @returns 排序后的数组
 */
export function sortArrValue(
  valArr: any[],
  origin: string[],
  target: string[]
) {
  const res: any[] = [];
  for (let i = 0; i < target.length; i++) {
    const v = target[i];
    const idx = origin.indexOf(v);
    if (idx !== -1) {
      res.push(valArr[idx]);
    }
  }
  return res;
}
