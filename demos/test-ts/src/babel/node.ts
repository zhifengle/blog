import * as types from '@babel/types';
import generate from '@babel/generator';

function createExpStmt() {
  var callee = types.memberExpression(
    types.identifier('console'),
    types.identifier('log')
  );
  var args = [types.numericLiteral(111)];
  var exp = types.callExpression(callee, args);
  var expressionStatement = types.expressionStatement(exp);
  console.log(generate(expressionStatement).code);
  return expressionStatement;
}
