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
    const tpl = template(`{
      FUNCTION_PARAMETER_CALLS
      DEFAULT_BODY
}`);
    node.body = tpl({
      FUNCTION_PARAMETER_CALLS: '11',
      DEFAULT_BODY: node.body.body,
    }) as types.BlockStatement;
    // const returnNode = node.body.body[0];
    //       node.body.body[0] = tpl({
    // DEFAULT_BODY:
    //       })
  },
};
const code = `function mirror(something) {
  consolo.log(22)
  return something
}`;
const ast = parse(code);
traverse(ast, visitor);
const transformedCode = generate(ast).code;
console.log(transformedCode);
