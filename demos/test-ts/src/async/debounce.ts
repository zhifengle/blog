import { Func } from "../types";

export function debounce(fn: Func, wait: number): Func {
  let timer: any = null;
  return function(this: any, ...args: any[]) {
    const ctx = this;
    clearTimeout(timer);
    timer = setTimeout(function() {
      fn.apply(ctx, args);
    });
  };
}
