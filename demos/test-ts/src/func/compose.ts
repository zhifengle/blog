import { Func } from '../types'

function compose(...args: any) {
  const len = args.length
  let count = len - 1
  let res: any = null
  return function innerFn(...innerArgs: any) {
    res = args[count].apply(this, innerArgs)
    if (count <= 0) {
      count = len - 1
      return res
    }
    count--
    return innerFn.call(null, res)
  }
}

const _pipe = (f: Func, g: Func) => (...args: any) =>
  g.call(null, f.apply(null, args))

const compose2 = (...args: Func[]) => args.reduceRight(_pipe)

const plus10 = (a: number) => a + 10
const square = (a: number) => a * a

const f = compose(square, plus10)
console.log(f(0))
