function isValid(s: string) {
  const stack = [];
  const map: { [prop: string]: string } = {
    '(': ')',
    '{': '}',
    '[': ']',
  };
  for (let i = 0; i < s.length; i++) {
    const c = s[i];
    if (map[c]) {
      stack.push(map[c]);
    } else if (c !== stack.pop()) {
      return false;
    }
  }
  return !stack.length;
}

test('s0020_valid_parentheses', () => {
  expect(isValid('{}()')).toBe(true);
});
