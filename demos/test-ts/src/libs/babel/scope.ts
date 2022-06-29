import { parse } from '@babel/parser';
import traverse from '@babel/traverse';

function scopDemo() {
  const jscode = `
function square(n) {
  return n * n;
}
function test() {
  var i = 11;
  i += 2;
  return i;
}
`;

  var ast = parse(jscode);
  traverse(ast, {
    FunctionDeclaration(path) {
      console.log('function name: ', path.node.id.name);
      path.scope.dump();
    },
  });

  // ------------------------------------------------------------
  // # FunctionDeclaration
  //  - n { constant: true, references: 2, violations: 0, kind: 'param' }
  // # Program
  //  - square { constant: true, references: 0, violations: 0, kind: 'hoisted' }
  //  - test { constant: true, references: 0, violations: 0, kind: 'hoisted' }
  // ------------------------------------------------------------
  // function name:  test
  // ------------------------------------------------------------
  // # FunctionDeclaration
  //  - i { constant: false, references: 1, violations: 1, kind: 'var' }
  // # Program
  //  - square { constant: true, references: 0, violations: 0, kind: 'hoisted' }
  //  - test { constant: true, references: 0, violations: 0, kind: 'hoisted' }
  // ------------------------------------------------------------
}

function bindingDemo() {
  const jscode = `
function a(){
    var a = 1;
    a = a + 1;
    return a;
}
function b(){
    var b = 1;
    var c = 2;
    b = b - c;
    return b;
}
`;
  traverse(parse(jscode), {
    BlockStatement(path) {
      const bindings = path.scope.bindings;
      // console.log(bindings);
      for (var key in bindings) {
        const b = bindings[key];
        // 引用次数
        b.references;
        // 引用路径
        b.referencePaths;
        // 是否会被修改
        b.constant;
        // 修改记录
        b.constantViolations;
      }
    },
  });
}
bindingDemo();
