import Parser from 'rss-parser';

export const MAGNET_PREFIX = 'magnet:?xt=urn:btih:';

export type Item = {
  magnet?: string;
  [key: string]: any;
} & Parser.Item;

export type MatchFn = (str: string, ...args: any) => boolean;

export type Pattern = string | MatchFn;

export type GetItemsFn = (
  rss: string,
  pattern?: Pattern,
  ...args: any
) => Promise<Item[]>;

export const patternMatch: MatchFn = (str, pattern: string) => {
  // @TODO flag
  const m = pattern.match(/^\/(.+)\/$/);
  if (m) {
    return new RegExp(m[1]).test(str);
  } else {
    return str.includes(pattern);
  }
};

export const identity: MatchFn = () => true;

export function getMatchFn(pattern: Pattern): MatchFn {
  if (!pattern) {
    return identity;
  }
  if (typeof pattern === 'string') {
    return (str) => patternMatch(str, pattern);
  }
  return pattern;
}

export function filterItems<T extends Item>(
  items: T[],
  pattern?: Pattern,
  key: string = 'title'
): T[] {
  if (!pattern) {
    return items;
  }
  const isMatch = getMatchFn(pattern);
  return items.filter((item) => {
    return isMatch(item[key]);
  });
}
