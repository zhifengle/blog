import { BPropType } from "../types";

const obj = {
  cookie: ""
};
export const fakeCookie = new Proxy(obj, {
  get(target: { cookie: string }, p: BPropType, receiver: any): any {
    const c = Reflect.get(target, p);
    if (p === "cookie") {
      return c.replace(/expires=.*?/, "");
    }
    return c;
  },
  set(
    target: { cookie: string },
    p: BPropType,
    value: any,
    receiver: any
  ): boolean {
    const c = Reflect.get(target, p);
    if (p === "cookie") {
      const v = c ? `${c};${value}` : value;
      Reflect.set(target, p, v);
    } else {
      Reflect.set(target, p, value);
    }
    return true;
  }
});
