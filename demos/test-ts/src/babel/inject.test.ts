import { inject } from './inject';

describe('VariableDeclaration', () => {
  test('init Literal', () => {
    let code = `var a = 1`;
    expect(inject(code)).toBe('var a = e_user_hook("a", 1, "var-init");');
    code = `const a = "abc"`;
    expect(inject(code)).toBe(`const a = e_user_hook("a", "abc", "var-init");`);
    code = `const a = obj.b.c;`;
    expect(inject(code)).toBe(
      `const a = e_user_hook("a", obj.b.c, "var-init");`
    );
  });
  test('init FunctionExpression', () => {
    let code = `
    var x = function test(a, b) {
  console.log(a, b);
};
    `;
    // expect(inject(code)).toBe(String);
  });
});

describe('ObjectExpression', () => {
  test('init ObjectExpression', () => {
    let code = `var obj = {
  a: 1,
  b: {
    c: x
  }
}`;
    expect(inject(code, { minified: true })).toBe(
      'var obj=e_user_hook("obj",{a:e_user_hook("a",1,"object-key-init"),b:{c:e_user_hook("c",x,"object-key-init")}},"var-init");'
    );
  });
});

describe('FunctionDeclaration', () => {
  test('init FunctionDeclaration', () => {
    let code = `function test(a, b = 1) {
  console.log(a, b);
}`;
    expect(inject(code, { minified: true })).toBe(
      `function test(a,b=1){e_user_hook("a",a,"function-parameter");e_user_hook("b",b,"function-parameter");console.log(a,b)}`
    );
  });
});
