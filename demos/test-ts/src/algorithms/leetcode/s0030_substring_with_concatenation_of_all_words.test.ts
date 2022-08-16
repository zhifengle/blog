function findSubstring(s: string, words: string[]): number[] {
  const res: number[] = [];
  if (!s || !words || words.length === 0) {
    return res;
  }
  const wordsLen = words.length;
  const len = words[0].length;
  const windowLen = len * wordsLen;
  const rec = (
    i: number,
    map: Record<string, number>,
    cnt: number
  ): boolean => {
    if (cnt === wordsLen) {
      return true;
    }
    const curWord = s.slice(i, len + i);
    let v = map[curWord];
    if (v) {
      map[curWord]--;
      let ret = rec(i + len, map, cnt + 1);
      map[curWord]++;
      return ret;
    }
    return false;
  };
  const m: Record<string, number> = {};
  for (let w of words) {
    m[w] = m[w] + 1 || 1;
  }
  for (let i = 0; i <= s.length - windowLen; i++) {
    if (rec(i, m, 0)) {
      res.push(i);
    }
  }

  return res;
}

test('s0030', () => {
  expect(findSubstring('barfoothefoobarman', ['foo', 'bar'])).toEqual([0, 9]);
  expect(
    findSubstring('wordgoodgoodgoodbestword', ['word', 'good', 'best', 'word'])
  ).toEqual([]);
  expect(
    findSubstring('barfoofoobarthefoobarman', ['bar', 'foo', 'the'])
  ).toEqual([6, 9, 12]);
});
