// 修改 insertBefore
const original = Element.prototype.insertBefore;
Element.prototype.insertBefore = function insertBefore(...args) {
  // @ts-ignore
  if (args[0] && args[0].tagName.includes('-')) {
    return;
  }
  const result = original.apply(this, args);

  return result;
};

export function hookObjProp(obj: any, prop: any) {
  var fakeObj = new Proxy(obj[prop], {
    // 对于 DOM 比如. Audio 需要这样写。避免 Illegal invocation
    get(target, prop, receiver) {
      const val = Reflect.get(target, prop);
      if (!!val && !!val.bind) {
        // 使用 bind 绑定 this 指向
        return val.bind(target);
      } else {
        return val;
      }
    },
    set(target, p, value, receiver) {
      return Reflect.set(target, p, value, target);
    },
    apply(target, thisArg, argArray) {
      if (!thisArg) {
        return Reflect.apply(target, obj, argArray);
      }
      return Reflect.apply(target, thisArg, argArray);
    },
  });
  Object.defineProperty(obj, prop, {
    get: function () {
      return fakeObj;
    },
    set: function (v) {
      console.log(`检测到修改 ${prop}`);
    },
  });
}
