// https://github.com/546669204/fuck-debugger-extensions/blob/master/inject.js

export function hookDebugger(fn: Function) {
  return function () {
    if (!arguments.length) {
      return fn.apply(this, arguments);
    }
    // 去除参数中的 debugger
    const args = [...arguments].map((arg) => {
      if (typeof arg === 'string' && arg.includes('debugger')) {
        return arg.replace(/debugger/, '');
      }
    });
    // xxFn.toString()  匹配里面是否有 debugger ??
    return fn.apply(this, args);
  };
}

export function hookDefault(window: Window & typeof globalThis) {
  // 在油猴脚本里面是 unsafeWindow
  // 这里是 iife 么 ??
  var ctor = window.Function.prototype.constructor;
  window.Function.prototype.constructor = hookDebugger(ctor);
  // @ts-ignore
  window.Function.prototype.constructor.name = ctor.name;
  window.Function.prototype.constructor.toString = ctor.toString.bind(ctor);

  var oldFn = Function;
  // @ts-ignore
  window.Function = hookDebugger(oldFn);
  // @ts-ignore
  window.Function.name = oldFn.name;
  window.Function.toString = oldFn.toString.bind(oldFn);

  var oldEval = eval;
  window.eval = hookDebugger(oldEval);
  // @ts-ignore
  window.eval.name = oldEval.name;
  window.eval.toString = oldEval.toString.bind(oldEval);

  // hook GeneratorFunction
  var oldGeneratorFunctionConstructor = Object.getPrototypeOf(
    function* () {}
  ).constructor;
  var newGeneratorFunctionConstructor = hookDebugger(
    oldGeneratorFunctionConstructor
  );
  newGeneratorFunctionConstructor.toString =
    oldGeneratorFunctionConstructor.toString.bind(
      oldGeneratorFunctionConstructor
    );
  // @ts-ignore
  newAsyncFunctionConstructor.name = oldAsyncFunctionConstructor.name;
  Object.defineProperty(
    oldGeneratorFunctionConstructor.prototype,
    'constructor',
    {
      value: newGeneratorFunctionConstructor,
      writable: false,
      configurable: true,
    }
  );

  // hook Async Function
  var oldAsyncFunctionConstructor = Object.getPrototypeOf(
    async function () {}
  ).constructor;
  var newAsyncFunctionConstructor = hookDebugger(oldAsyncFunctionConstructor);
  // @ts-ignore 重置名字
  newAsyncFunctionConstructor.name = oldAsyncFunctionConstructor.name;
  newAsyncFunctionConstructor.toString =
    oldAsyncFunctionConstructor.toString.bind(oldAsyncFunctionConstructor);
  Object.defineProperty(oldAsyncFunctionConstructor.prototype, 'constructor', {
    value: newAsyncFunctionConstructor,
    writable: false,
    configurable: true,
  });

  // hook dom
  // ?? 这里直接复制的没有测试
  var oldSetAttribute = window.Element.prototype.setAttribute;
  window.Element.prototype.setAttribute = function (name, value) {
    if (typeof value == 'string') value = value.replace(/debugger/g, '');
    // 向上调用
    oldSetAttribute.call(this, name, value);
  };
  var oldContentWindow = Object.getOwnPropertyDescriptor(
    HTMLIFrameElement.prototype,
    'contentWindow'
  ).get;
  Object.defineProperty(window.HTMLIFrameElement.prototype, 'contentWindow', {
    get() {
      var newV = oldContentWindow.call(this);
      if (!newV.inject) {
        newV.inject = true;
        hookDefault.call(newV, newV);
      }
      return newV;
    },
  });
}
