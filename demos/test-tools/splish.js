const str = 'narvaja'
const a = 10
const res = []
function genVaildNum(s) {
  let m = 5 * 0xa + s
  if (m > 0x39) {
    m -= 0xa
    while (m > 0x39) {
      m -= 0xa
    }
  } else if (m < 0x30) {
    m += 0xa
    while (m < 0x30) {
      m += 0xa
    }
  }
  return m
}
for (let i = 0; i < str.length; i++) {
  const num = str.charCodeAt(i)
  const s = parseInt(num / a)
  let m = (num % a ^ i) + 2
  if (m > 10) {
    m -= 10
  }
  m = genVaildNum(m)
  // m = 5 * 0xa + m
  // if (m > 0x39) {
  //   m -= 0xa
  // } else if (m < 0x30) {
  //   m += 0xa
  // }
  res.push(String.fromCharCode(m))
}
console.log(res.join(''))
