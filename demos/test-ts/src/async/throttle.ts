import { Func } from "../types";

export function throttle(fn: Func, delay: number): Func {
  let last: number = 0;
  return function(this: any, ...args: any[]) {
    let now: number = +new Date();
    if (now - last > delay || !delay) {
      fn.apply(this, args);
    }
  };
}
