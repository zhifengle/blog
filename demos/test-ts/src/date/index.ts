type IDateType = 'year' | 'month' | 'date';

export function genDateSequence(type: IDateType, start: string, end: string) {
  const genNums = (s: number, e: number, gap: number = 1) => {
    const arr = [];
    for (let i = s; i <= e; i += gap) {
      arr.push(i);
    }
    return arr;
  };
  const fillZero = (num: number) => {
    if (num === 0) return '0';
    const strNum = '' + num;
    if (strNum.length === 1) return '0' + strNum;
    return strNum;
  };
  const startDate = new Date(start);
  const endDate = new Date(end);
  if (startDate > endDate) {
    throw 'Start date bigger than end date';
  }
  if (type === 'year') {
    return genNums(startDate.getFullYear(), endDate.getFullYear()).map(
      (n) => '' + n
    );
  }
  if (type === 'month') {
    const startMonth = startDate.getMonth();
    const endMonth =
      (endDate.getFullYear() - startDate.getFullYear()) * 12 +
      endDate.getMonth();
    return genNums(startMonth, endMonth).map((n) => {
      const d = new Date(startDate.getFullYear(), n);
      return `${d.getFullYear()}-${fillZero(d.getMonth() + 1)}`;
    });
  }
  if (type === 'date') {
    const oneDay = 1000 * 60 * 60 * 24;
    return genNums(+startDate, +endDate, oneDay).map((n) => {
      const d = new Date(n);
      return `${d.getFullYear()}-${fillZero(d.getMonth() + 1)}-${fillZero(
        d.getDate()
      )}`;
    });
  }
}

export function formatDate(time: any, fmt: string = 'yyyy-MM-dd') {
  const date = new Date(time);
  var o: any = {
    'M+': date.getMonth() + 1, //月份
    'd+': date.getDate(), //日
    'h+': date.getHours(), //小时
    'm+': date.getMinutes(), //分
    's+': date.getSeconds(), //秒
    'q+': Math.floor((date.getMonth() + 3) / 3), //季度
    S: date.getMilliseconds(), //毫秒
  };
  if (/(y+)/i.test(fmt)) {
    fmt = fmt.replace(
      RegExp.$1,
      (date.getFullYear() + '').substr(4 - RegExp.$1.length)
    );
  }
  for (var k in o) {
    if (new RegExp('(' + k + ')', 'i').test(fmt)) {
      fmt = fmt.replace(
        RegExp.$1,
        RegExp.$1.length == 1 ? o[k] : ('00' + o[k]).substr(('' + o[k]).length)
      );
    }
  }
  return fmt;
}
