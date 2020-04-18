import { Func, UnaryFunc } from "../types";
import { identity } from "../common/identity";

export function pipe<T, R>(fns: UnaryFunc<T, R>[]): UnaryFunc<T, R> {
  if (fns.length === 0) {
    return identity as UnaryFunc<any, any>;
  }
  if (fns.length === 1) {
    return fns[0];
  }

  return function piped(input: T): R {
    return fns.reduce(
      (prev: any, fn: UnaryFunc<T, R>) => fn(prev),
      input as any
    );
  };
}
