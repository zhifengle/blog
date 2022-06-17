import { Lexer } from '../lexer';
import {
  CLOSE_BRACKET,
  CLOSE_PAREN,
  EOF,
  LINE_BREAK,
  OPEN_BRACKET,
  OPEN_PAREN,
  SLASH,
} from '../types';
import type { TokenType } from '../types';

function expectTokens(lexer: Lexer, expected: TokenType[]) {
  for (const token of expected) {
    expect(lexer.nextToken()).toEqual(token);
  }
  expect(lexer.nextToken()).toEqual(EOF);
}
describe('bbcode lexer', () => {
  test('multi language', () => {
    const s = '啊aあ\n)';
    const expected = ['啊aあ', LINE_BREAK, CLOSE_PAREN];
    expectTokens(new Lexer(s), expected);
  });
  test('ascii string', () => {
    const s = 'bbcode \t\n[]()';
    const expected = [
      'bbcode \t',
      LINE_BREAK,
      OPEN_BRACKET,
      CLOSE_BRACKET,
      OPEN_PAREN,
      CLOSE_PAREN,
    ];
    expectTokens(new Lexer(s), expected);
  });
  test('bbcode string', () => {
    const s = '[s]bbcode\n[/s]';
    const expected = [
      OPEN_BRACKET,
      's',
      CLOSE_BRACKET,
      'bbcode',
      LINE_BREAK,
      OPEN_BRACKET,
      SLASH,
      's',
      CLOSE_BRACKET,
    ];
    expectTokens(new Lexer(s), expected);
  });
});
