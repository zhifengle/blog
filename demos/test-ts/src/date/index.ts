type IDateType = 'year' | 'month' | 'date'

export function genDateSequence(type: IDateType, start: string, end: string) {
  const genNums = (s: number, e: number, gap: number = 1) => {
    const arr = []
    for (let i = s; i <= e; i += gap) {
      arr.push(i)
    }
    return arr
  }
  const fillZero = (num: number) => {
    if (num === 0) return '0'
    const strNum = '' + num
    if (strNum.length === 1) return '0' + strNum
    return strNum
  }
  const startDate = new Date(start)
  const endDate = new Date(end)
  if (startDate > endDate) {
    throw 'Start date bigger than end date'
  }
  if (type === 'year') {
    return genNums(startDate.getFullYear(), endDate.getFullYear()).map(
      (n) => '' + n
    )
  }
  if (type === 'month') {
    const startMonth = startDate.getMonth()
    const endMonth =
      (endDate.getFullYear() - startDate.getFullYear()) * 12 +
      endDate.getMonth()
    return genNums(startMonth, endMonth).map((n) => {
      const d = new Date(startDate.getFullYear(), n)
      return `${d.getFullYear()}-${fillZero(d.getMonth() + 1)}`
    })
  }
  if (type === 'date') {
    const oneDay = 1000 * 60 * 60 * 24
    return genNums(+startDate, +endDate, oneDay).map((n) => {
      const d = new Date(n)
      return `${d.getFullYear()}-${fillZero(d.getMonth() + 1)}-${fillZero(
        d.getDate()
      )}`
    })
  }
}
