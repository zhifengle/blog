import { parse, ParseResult, traverse, Visitor } from '@babel/core';
import template from '@babel/template';
import generate from '@babel/generator';
import * as types from '@babel/types';

const visitor: Visitor = {
  FunctionDeclaration(path) {
    const node = path.node;
    if (!node.params?.length) {
      return;
    }
    const params = node.params;
    params.forEach((p) => {
      console.log(p);
    });
  },
};
const code = `function test(a, b, c = 1, ...args) {
  console.log(a, b);
};`;

const ast = parse(code);
traverse(ast, visitor);
