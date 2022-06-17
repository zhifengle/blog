import type { TokenType, TextNode } from './types';
import {
  OPEN_BRACKET,
  EOF,
  CLOSE_BRACKET,
  OPEN_PAREN,
  CLOSE_PAREN,
  ASSIGN,
  LINE_BREAK,
  SLASH,
} from './types';

const SPECIAL_CHARS = [
  OPEN_BRACKET,
  CLOSE_BRACKET,
  OPEN_PAREN,
  CLOSE_PAREN,
  ASSIGN,
  LINE_BREAK,
  SLASH,
];

export class Lexer {
  private readonly input: string;
  private position: number;
  private readPosition: number;
  private ch: string;

  constructor(input: string) {
    this.input = input;
    this.position = 0;
    this.readPosition = 0;
    this.ch = '';
    this.readChar();
  }

  nextToken(): TokenType {
    if (SPECIAL_CHARS.includes(this.ch)) {
      const token = this.ch;
      this.readChar();
      return token;
    }
    if (this.ch !== '') {
      return this.readText();
    } else {
      return EOF;
    }
  }

  readChar() {
    if (this.readPosition >= this.input.length) {
      this.ch = '';
    } else {
      this.ch = this.input[this.readPosition];
    }
    this.position = this.readPosition;
    this.readPosition += 1;
  }

  peekChar(): string {
    if (this.readPosition >= this.input.length) {
      return '';
    } else {
      return this.input[this.readPosition];
    }
  }

  readText(): TextNode {
    const start = this.position;
    let end = this.position;
    while (this.ch && !SPECIAL_CHARS.includes(this.ch)) {
      this.readChar();
      end += 1;
    }
    return this.input.slice(start, end);
  }
}
