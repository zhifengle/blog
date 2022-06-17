import { Lexer } from './lexer';
import type { TokenType, VNodeTypes } from './types';

export class Parser {
  l: Lexer;
  curToken: TokenType;
  peekToken: TokenType;
  constructor(l: Lexer) {
    this.l = l;
    this.curToken = l.nextToken();
    this.peekToken = l.nextToken();
  }
  nextToken() {
    this.curToken = this.peekToken;
    this.peekToken = this.l.nextToken();
  }
  parseNodes() {
    const nodes: TokenType[] = [];
    while (this.curToken) {}
  }
  parseNode(): VNodeTypes {
    switch (this.curToken) {
      case '(':
      case '[':
      default:
        break;
    }
    return '';
  }
  // (bgm38)
  parseStickerNode(): VNodeTypes {
    let str = this.curToken;
    if (/bgm\d+/.test(this.peekToken)) {
      this.nextToken();
      str += this.curToken;
      if (this.peekToken === ')') {
        this.nextToken();
        this.nextToken();
        const id = this.curToken.slice(3);
        return {
          type: 'img',
          props: {
            'sticker-id': id,
            smileid: id,
            alt: `(bgm${id})`,
          },
        };
      }
    }
    return str;
  }
  parseTextNode(): VNodeTypes {
    return '';
  }
  parseBBCodeNode(): VNodeTypes {
    return '';
  }
}
