import { CodeNodeTypes } from './types';

const INVALID_NODE_MSG = 'invalid node';
const INVALID_STICKER_NODE = 'invalid sticker node';

export const BBCODE_REGEXP = /^\[([a-z]+)(=.+?)?\](.+?)\[\/([a-z]+)\]/;

type CheckCharFn = (str: string) => boolean;

export class Parser {
  private pos: number;
  // 记录上下文
  private ctxStack: { startIdx: number }[];
  constructor(private readonly input: string) {
    this.pos = 0;
    this.ctxStack = [];
  }
  parseNodes(): CodeNodeTypes[] {
    const nodes: CodeNodeTypes[] = [];
    while (true) {
      if (this.eof() || this.startsWith('[/')) {
        break;
      }
      nodes.push(this.parseNode());
    }
    return nodes;
  }
  parseNode(): CodeNodeTypes {
    this.enterNode();
    let node: CodeNodeTypes;
    try {
      switch (this.curChar()) {
        case '(':
          node = this.parseStickerNode();
          break;
        case '[':
          node = this.parseBBCodeNode();
          break;
        default:
          node = this.parseText();
      }
      this.exitNode();
    } catch (error) {
      // console.log(this.input.slice(this.pos), this.curChar());
      // console.log(error);
      if (
        error.message === INVALID_NODE_MSG ||
        error.message === INVALID_STICKER_NODE
      ) {
        const ctx = this.ctxStack.pop();
        node = this.input.slice(ctx.startIdx, this.pos);
      }
    }
    return node;
  }
  // 解析 (bgm38) (bgm1)
  parseStickerNode(): CodeNodeTypes {
    this.consumeChar();
    if (!this.startsWith('bgm')) {
      throw new Error(INVALID_STICKER_NODE);
    }
    this.pos += 3;
    const id = this.consumeWhile((c) => !isNaN(+c));
    if (!id) {
      throw new Error(INVALID_STICKER_NODE);
    }
    const c = this.consumeChar();
    if (c !== ')') {
      throw new Error(INVALID_STICKER_NODE);
    }
    return {
      type: 'img',
      props: {
        smileid: id,
        alt: `(bgm${id})`,
      },
    };
  }
  // @TODO 暂时只支持 Bangumi 的 bbcode; 不支持 [style size="30px"]Large Text[/style]
  parseBBCodeNode(): CodeNodeTypes {
    let c = this.consumeChar();
    const openTag = this.parseTagName();
    c = this.consumeChar();
    if (![']', '='].includes(c)) {
      throw new Error(INVALID_NODE_MSG);
    }
    let prop: string;
    if (c === '=') {
      prop = this.consumeWhile((c) => c !== ']');
      c = this.consumeChar();
    }
    let children = this.parseNodes();

    this.validateStartStr('[/');
    const closeTag = this.parseTagName();
    if (openTag !== closeTag) {
      throw new Error(INVALID_NODE_MSG);
    }
    this.validateStartStr(']');
    const n: CodeNodeTypes = {
      type: openTag,
      children,
    };
    if (prop) {
      n.props = {
        [openTag]: prop,
      };
    }
    return n;
  }
  parseTagName(): string {
    const tag = this.consumeWhile((c) => /[a-zA-Z]/.test(c));
    if (!tag) {
      throw new Error(INVALID_NODE_MSG);
    }
    return tag;
  }
  parseText(): string {
    return this.consumeWhile((c) => !['(', '['].includes(c));
  }
  consumeWhile(checkFn: CheckCharFn) {
    let result = '';
    while (!this.eof() && checkFn(this.curChar())) {
      result += this.consumeChar();
    }
    return result;
  }
  consumeChar(): string {
    const cur = this.input[this.pos];
    this.pos += 1;
    return cur;
  }
  consumeWhitespace() {
    this.consumeWhile((c) => /\s/.test(c));
  }
  getChildrenStr(tag: string) {
    let result = '';
    while (!this.eof()) {
      result += this.consumeChar();
    }
    return result;
  }
  curChar(): string {
    return this.input[this.pos];
  }
  startsWith(pattern: string | RegExp): boolean {
    if (typeof pattern === 'string') {
      return this.input.slice(this.pos).startsWith(pattern);
    } else {
      return pattern.test(this.input.slice(this.pos));
    }
  }
  eof(): boolean {
    return this.pos >= this.input.length;
  }
  enterNode() {
    this.ctxStack.push({
      startIdx: this.pos,
    });
  }
  exitNode() {
    this.ctxStack.pop();
  }
  validateStartStr(str: string): boolean {
    let count = 0;
    while (count < str.length) {
      if (this.consumeChar() !== str[count]) {
        throw new Error(INVALID_NODE_MSG);
      }
      count++;
    }
    return true;
  }
}
