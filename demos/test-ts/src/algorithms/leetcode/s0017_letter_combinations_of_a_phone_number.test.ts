function letterCombinations(digits: string): string[] {
  let res: string[] = [];
  const lookup = [
    '',
    '',
    'abc',
    'def',
    'ghi',
    'jkl',
    'mno',
    'pqrs',
    'tuv',
    'wxyz',
  ];
  for (const strNum of digits) {
    const i = +strNum;
    if (res.length === 0) {
      for (const c of lookup[i]) {
        res.push(c);
      }
      continue;
    }
    const r: string[] = [];
    for (const s of res) {
      for (const c of lookup[i]) {
        r.push(s + c);
      }
    }
    res = r;
  }
  return res;
}

test('s0017_letter_combinations_of_a_phone_number', () => {
  expect(letterCombinations('23')).toEqual([
    'ad',
    'ae',
    'af',
    'bd',
    'be',
    'bf',
    'cd',
    'ce',
    'cf',
  ]);
});
