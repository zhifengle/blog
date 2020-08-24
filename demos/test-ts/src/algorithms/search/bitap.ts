function createPatternAlphabet(pattern: string) {
  let mask: any = {}

  for (let i = 0, len = pattern.length; i < len; i += 1) {
    const char = pattern.charAt(i)
    mask[char] = (mask[char] || 0) | (1 << (len - i - 1))
  }

  return mask
}

function bitap(text: string, pattern: string) {
  const LIMIT_LEN = 63
  const m = pattern.length
  const patternMask: any = []
  let R = ~1
  if (!m) return -1
  if (m > LIMIT_LEN) return -1
  // for (let i = 0; i <= LIMIT_LEN; ++i) {
  //   patternMask[i] = ~0
  // }
  for (let j = 0; j < m; j++) {
    patternMask[pattern[j].charCodeAt(0)] &= ~(1 << j)
  }
  for (let i = 0; i < text.length; i++) {
    const t = patternMask[text[i].charCodeAt(0)] || ~0
    R |= t
    R <<= 1
    if ((R & (1 << m)) === 0) {
      return i - m + 1
    }
  }
  return -1
}

console.log(bitap('abc', 'c'))
