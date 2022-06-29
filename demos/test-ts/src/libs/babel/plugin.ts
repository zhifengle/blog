import { parse, traverse } from '@babel/core';
import generate from '@babel/generator';

// ast.md

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
