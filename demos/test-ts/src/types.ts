export type BPropType = string | number | symbol;

export type Func = (...args: any) => any;

export interface UnaryFunc<T, R> {
  (source: T): R;
}
