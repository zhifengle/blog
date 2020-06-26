// https://github.com/mqyqingfeng/Blog/issues/11
export function call2(ctx: any, ...args: any) {
  const context = ctx || window
  context.fn = this
  const innerArgs: any[] = []
  for (let i = 1; i < args.length; i++) {
    innerArgs.push('args[' + i + ']')
  }

  const result = eval('context.fn(' + args + ')')

  delete context.fn
  return result
}

export function bind2(ctx: any, ...args: any) {
  if (typeof this !== 'function') {
    throw new Error('not fun')
  }
  const self = this
  const nop: any = function () {}
  const _bind = function (...innerArgs: any) {
    return self.apply(this instanceof nop ? this : ctx, [...args, ...innerArgs])
  }
  nop.prototype = this.prototype
  _bind.prototype = new nop()
  return _bind
}

export function instance_of(l: any, r: any): boolean {
  let o = r.prototype
  let p = Object.getPrototypeOf(l)
  while (true) {
    if (!p) return false
    if (o === p) return true
    p = Object.getPrototypeOf(p)
  }
}
