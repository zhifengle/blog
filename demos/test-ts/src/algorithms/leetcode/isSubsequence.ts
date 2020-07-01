export function isSubsequence<T>(s: T[], t: T[]): boolean {
  if (!s.length) return true
  let ps = 0
  let pt = 0
  while (pt < t.length) {
    if (s[ps] === t[pt]) {
      ps++
      pt++
      if (ps >= s.length) return true
    } else {
      pt++
    }
  }
  return false
}
