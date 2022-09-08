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
function isValid2(s: string) {
  const stack = [];
  const m: Map<string, string> = new Map([
    ['(', ')'],
    ['{', '}'],
    ['[', ']'],
  ]);
  for (let i = 0, len = s.length; i < len; i++) {
    const c = s[i];
    if (m.has(c)) {
      stack.push(m.get(c));
    } else if (c !== stack.pop()) {
      return false;
    }
  }
  return !stack.length;
}

test('s0020_valid_parentheses', () => {
  expect(isValid2('{}()')).toBe(true);
});
