import { Watcher } from './watcher';

let uid = 0;

// 消息中心
export class Dep {
  static target: Watcher | null;
  id: number;
  subs: Array<Watcher>;
  constructor() {
    this.id = uid++;
    this.subs = [];
  }
  addSub(sub: Watcher) {
    this.subs.push(sub);
  }
  // @TODO 省略了这个实现
  // removeSub()

  depend() {
    if (Dep.target) {
      Dep.target.addDep(this);
    }
  }

  // Vue 源码有判断环境
  notify() {
    this.subs.forEach((sub) => sub.update());
  }
}
Dep.target = null;
