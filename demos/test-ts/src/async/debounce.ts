import { Func } from '../types'

export function debounce(fn: Func, wait: number): Func {
  let timer: any = null
  return function (this: any, ...args: any[]) {
    const ctx = this
    clearTimeout(timer)
    timer = setTimeout(function () {
      fn.apply(ctx, args)
    }, wait)
  }
}

export function debouncePromise(inner: Func, ms = 300) {
  let timer: any = null
  let resolves: any[] = []

  return function (...args: any) {
    clearTimeout(timer)
    timer = setTimeout(() => {
      let result = inner(...args)
      resolves.forEach((r) => r(result))
      resolves = []
    }, ms)

    return new Promise((r) => resolves.push(r))
  }
}
