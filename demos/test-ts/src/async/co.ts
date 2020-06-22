// @TODO gen type
export function co(gen: any, ...args: any): Promise<any> {
  const ctx = this
  return new Promise(function (resolve, reject) {
    function fulfilled(value: any) {
      try {
        step(gen.next(value))
      } catch (e) {
        reject(e)
      }
    }
    function rejected(value: any) {
      try {
        step(gen['throw'](value))
      } catch (e) {
        reject(e)
      }
    }

    function step(result: any) {
      result.done
        ? resolve(result.value)
        : new Promise(function (resolve) {
            resolve(result.value)
          }).then(fulfilled, rejected)
    }
    step((gen = gen.apply(ctx, args)).next())
  })
}
