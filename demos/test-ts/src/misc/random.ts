export function genRandomStr(len: number): string {
  return Array.apply(null, Array(len))
    .map(function () {
      return (function (chars) {
        return chars.charAt(Math.floor(Math.random() * chars.length));
      })('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789');
    })
    .join('');
}

export function randomNum(max: number, min: number): number {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

export const convertNumberToBinary = (num: number) => {
  return (num >>> 0).toString(2);
};

/**
 * 转换数字为百分比
 * @param num
 * @returns
 */
export function numToPercent(num: number) {
  return Number(num || 0).toLocaleString(undefined, {
    style: 'percent',
    minimumFractionDigits: 2,
  });
}

/**
 * 四舍五入数据、保留位数
 * @param num
 * @param len
 * @returns
 */
export function roundNum(num: number, len: number = 2) {
  //@ts-ignore
  return +(Math.round(num + `e+${len}`) + `e-${len}`);
}
