function genCode(s) {
  let r = 0
  let v3 = 0
  let v1 = 0
  for (let i = 0; i < s.length; i++) {
    v1 = 10
    v3 = s[i].charCodeAt(0) - 48
    r = v3 + v1 * r
  }
  return r ^ 0x1234
}
// 0x5696
function genSerial(num) {
  let r = ''
  let v2 = num ^ 0x1234
  console.log(v2)
  return r
}
console.log(genCode('98989898').toString(16))
genSerial(0x5696)
