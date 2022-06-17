import { Lexer } from './lexer';
import { EOF, TokenType } from './types';

const INVALID_NODE_MSG = 'invalid node';

export const BBCODE_REGEXP = /^\[([a-z]+)(=.+?)?\](.+?)\[\/([a-z]+)\]/;

type CheckCharFn = (str: string) => boolean;
type AttrMap = Record<string, string>;

export type VNode = {
  type: string;
  props?: Record<string, string>;
  children?: NodeTypes[];
};

export type NodeTypes = string | VNode;

const OPEN_CHARS = ['(', '['];
const END_CHARS = [')', ']', '='];

export class Parser {
  private pos: number;
  // 记录当前解析节点的开始
  private start: number;
  constructor(private readonly input: string) {
    this.pos = 0;
    this.start = 0;
  }
  parseNodes(): NodeTypes[] {
    const nodes: NodeTypes[] = [];
    while (!this.eof()) {
      let node: NodeTypes;
      try {
        node = this.parseNode();
      } catch (error) {
        if (error.message === INVALID_NODE_MSG) {
          node = this.input.slice(this.start, this.pos);
          this.start = this.pos;
        }
      }
      if (node) {
        if (typeof node !== 'string') {
          node && nodes.push(node);
        } else {
          const prevNode = nodes[nodes.length - 1];
          if (typeof prevNode === 'string') {
            nodes[node.length - 1] = prevNode + node;
          } else {
            node && nodes.push(node);
          }
        }
      }
    }
    return nodes;
  }
  parseNode(): NodeTypes {
    this.enterNode();
    switch (this.curChar()) {
      case '(':
        return this.parseStickerNode();
      case '[':
        return this.parseBBCodeNode();
      default:
        return this.parseText();
    }
  }
  // (bgm38)
  parseStickerNode(): NodeTypes {
    // if (!this.startsWith('(bgm')) {
    //   throw new Error(INVALID_NODE_MSG);
    // }
    // this.pos += 4;
    this.validateStartStr('(bgm');
    const id = this.consumeWhile((c) => !isNaN(+c));
    if (!id) {
      throw new Error(INVALID_NODE_MSG);
    }
    const c = this.consumeChar();
    if (c !== ')') {
      throw new Error(INVALID_NODE_MSG);
    }
    return {
      type: 'img',
      props: {
        'sticker-id': id,
        smileid: id,
        alt: `(bgm${id})`,
      },
    };
  }
  parseTextNode(): NodeTypes {
    let str = '';
    return str;
  }
  // @TODO 暂时只支持 Bangumi 的 bbcode
  parseBBCodeNode(): NodeTypes {
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
    // let closeTagIdx = this.input.slice(this.pos).lastIndexOf(`[/${openTag}]`);
    // if (closeTagIdx === -1) {
    //   throw new Error(INVALID_NODE_MSG);
    // }
    // closeTagIdx += this.pos;
    // let childrenStr = this.input.slice(this.pos, closeTagIdx);
    // this.pos += childrenStr.length;

    // let closeTagIdx = this.input.indexOf(`[/${openTag}]`, this.pos);
    // if (closeTagIdx === -1) {
    //   // throw new Error(INVALID_NODE_MSG);
    //   this.pos = this.input.length;
    //   return this.input.slice(this.start);
    // }
    // let childrenStr = this.input.slice(this.pos, closeTagIdx);
    // this.pos += childrenStr.length;

    let childrenStr = this.consumeWhile((c) => c !== '[');
    // 使用当前对象有问题。暂时采取 new 一个对象
    const children = new Parser(childrenStr).parseNodes();
    this.validateStartStr('[');
    this.validateStartStr('/');
    const closeTag = this.parseTagName();
    if (openTag !== closeTag) {
      throw new Error(INVALID_NODE_MSG);
    }
    this.validateStartStr(']');
    const n: NodeTypes = {
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
    this.start = this.pos;
  }
  exitNode() {}
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
