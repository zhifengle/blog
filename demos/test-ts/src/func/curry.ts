import { Func } from "../types";

function subCurry(fn: Func, ...args: any[]) {
  return function (...innerArgs: any[]) {
    return fn.apply(this, [...args, ...innerArgs]);
  };
}

export function curry(fn: Func, len = fn.length as number) {
  return function (...innerArgs: any[]) {
    if (innerArgs.length < len) {
      return curry(
        subCurry.apply(this, [fn, ...innerArgs]),
        len - innerArgs.length
      );
    } else {
      fn.apply(this, innerArgs);
    }
  };
}
