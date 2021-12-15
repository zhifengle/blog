import { transformSync, parse, traverse } from '@babel/core';
import generate from '@babel/generator';
import { isFunctionExpression } from '@babel/types';

const code = 'const n = 1';
function demo() {
  const code = 'const n = 1';

  const ast = parse(code);

  // transform the ast
  traverse(ast, {
    enter(path) {
      // in this example change all the variable `n` to `x`
      if (path.isIdentifier({ name: 'n' })) {
        path.node.name = 'x';
      }
    },
  });

  // generate code <- ast
  const output = generate(ast, {}, code);
  console.log(output.code); // 'const x = 1;'
}

function myInject(code: string) {
  const ast = parse(code);
  // { } 内容是 visitor
  traverse(ast, {
    // 变量声明
    VariableDeclaration(path) {
      const node = path.node;
      if (!node.declarations?.length) {
        return;
      }
      for (let variableDeclarator of node.declarations) {
        if (!variableDeclarator.init) {
          continue;
        }
        if (isFunctionExpression(variableDeclarator.init)) {
          continue;
        }
      }
    },
  });
  // return generate.default(ast).code;
}

myInject(code);
