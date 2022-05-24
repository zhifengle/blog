function generateParenthesis(n: number): string[] {
  if (n < 1) {
    return [];
  }
  const result: string[] = [];
  const dfs = (
    n: number,
    left: number,
    right: number,
    result: string[],
    path: string
  ) => {
    if (left === n && right === n) {
      result.push(path);
      return;
    }
    if (left < n) {
      dfs(n, left + 1, right, result, path + '(');
    }
    if (right < left) {
      dfs(n, left, right + 1, result, path + ')');
    }
  };
  dfs(n, 0, 0, result, '');
  return result;
}

test('s0022_generate_parentheses', () => {
  expect(generateParenthesis(3)).toEqual([
    '((()))',
    '(()())',
    '(())()',
    '()(())',
    '()()()',
  ]);
});
