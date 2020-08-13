// @TODO type
export function uniqueByKey(arr: any[], key: string) {
  return arr.filter((item: any, pos: number) => {
    return (
      arr.findIndex((el: any) => {
        return el[key] === item[key]
      }) === pos
    )
  })
}
