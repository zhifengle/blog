import { Dep } from './dep';

type IFunc = (...args: any) => any;

// export interface Watcher {
//   depIds: Record<number, Dep>;
//   vm: any;
//   cb: any;
//   expOrFn: any;
//   val: any;

//   update(): void;
//   addDep(dep: Dep): void;
//   run(): void;
//   get(): any;
// }

// 订阅者
export class Watcher {
  depIds: Record<number, Dep>;
  vm: any;
  cb: IFunc;
  expOrFn: any;
  val: any;
  constructor(vm: any, expOrFn: any, cb: IFunc) {
    this.depIds = {}; // hash储存订阅者的id,避免重复的订阅者
    this.vm = vm; // 被订阅的数据一定来自于当前Vue实例
    this.cb = cb; // 当数据更新时想要做的事情
    this.expOrFn = expOrFn; // 被订阅的数据
    this.val = this.get(); // 维护更新之前的数据
  }
  update() {
    this.run();
  }
  addDep(dep: Dep) {
    // 如果在depIds的hash中没有当前的id,可以判断是新Watcher,因此可以添加到dep的数组中储存
    // 此判断是避免同id的Watcher被多次储存
    if (!this.depIds.hasOwnProperty(dep.id)) {
      dep.addSub(this);
      this.depIds[dep.id] = dep;
    }
  }
  run() {
    const val = this.get();
    if (val !== this.val) {
      this.val = val;
      this.cb.call(this.vm, val);
    }
  }
  get() {
    Dep.target = this;
    const val = this.vm._data[this.expOrFn];

    Dep.target = null;
    return val;
  }
}
