function longestCommonPrefix(strs: string[]): string {
  if (!strs || strs.length === 0) {
    return '';
  }
  return strs.reduce((prev, next) => {
    let i = 0;
    while (prev[i] && next[i] && prev[i] === next[i]) {
      i++;
    }
    return prev.slice(0, i);
  });
}

test('s0014_longest_common_prefix', () => {
  expect(longestCommonPrefix(['flower', 'flow', 'flight'])).toBe('fl');
  expect(longestCommonPrefix(['', '', ''])).toBe('');
  expect(longestCommonPrefix(['flower', 'flower', 'flower'])).toBe('flower');
});
