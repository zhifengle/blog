// https://www.barretlee.com/blog/2016/08/23/javascript-sandbox/
// https://zhuanlan.zhihu.com/p/58602800

const sandboxProxies = new WeakMap();
function compileCode(src: string) {
  src = 'with (sandbox) {' + src + '}';
  const code = new Function('sandbox', src);
  function has() {
    return true;
  }
  function get(target: any, key: any) {
    // 会被绕过
    //  Object.keys(Array.prototype[Symbol.unscopables]);
    if (key === Symbol.unscopables) return undefined;
    return target[key];
  }
  return function (sandbox: any) {
    if (!sandboxProxies.has(sandbox)) {
      const sandboxProxy = new Proxy(sandbox, { has, get });
      sandboxProxies.set(sandbox, sandboxProxy);
    }
    return code(sandboxProxies.get(sandbox));
  };
}

function compileCode2(code: string) {
  code = 'with (sandbox) {' + code + '}';
  const fn = new Function('sandbox', code);
  return (sandbox: any) => {
    const proxy = new Proxy(sandbox, {
      has(target, key) {
        return true; // 欺骗，告知属性存在
      },
      get(target, key, receiver) {
        // 加固，防止逃逸
        if (key === Symbol.unscopables) {
          return undefined;
        }
        Reflect.get(target, key, receiver);
      },
    });
    return fn(proxy);
  };
}
