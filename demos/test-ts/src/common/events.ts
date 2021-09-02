// https://github.com/developit/mitt/blob/main/src/index.ts

// all!.get('xx')  表示 all 肯定不是 null

// @TODO symbol 使用 map 来存储
export type EventType = string;

export type Handler<T = unknown> = (event: T) => void;
export type EventHandlerList<T = unknown> = Array<Handler<T>>;

// A map of event types and their corresponding event handlers.
export type EventHandlerMap<Events extends Record<EventType, unknown>> = Record<
  keyof Events,
  EventHandlerList<Events[keyof Events]>
>;

// https://stackoverflow.com/questions/36382299/is-it-possible-to-define-a-type-string-literal-union-within-a-class-in-typescr
module EventEmitter {
  export type GenericEventHandler<T = unknown> = Handler<T[keyof T]>;
}
// 下面的方法可以使用。
// 不过按照这种写法，似乎不用写在 module 里面也行
// EventEmitter.GenericEventHandler<Events>

export class EventEmitter<Events extends Record<EventType, unknown>> {
  events: EventHandlerMap<Events>;
  constructor() {
    this.events = {} as EventHandlerMap<Events>;
  }
  // <Key extends keyof Events> 解决  this.events[type] 报错
  on<Key extends keyof Events>(type: Key, handler: Handler) {
    const handlers: Array<Handler> | undefined = this.events[type];
    if (!handlers) {
      this.events[type] = [handler];
    } else {
      handlers.push(handler);
    }
  }
  // @TODO handler 的类型
  off<Key extends keyof Events>(type: Key, handler: Handler) {
    const handlers: Array<Handler> | undefined = this.events[type];
    if (handlers) {
      if (handler) {
        // 为什么会 >>> 0 ?? ;  1 >>> 0  ---> 1
        handlers.splice(handlers.indexOf(handler) >>> 0, 1);
      } else {
        this.events[type] = [];
      }
    }
  }
  emit<Key extends keyof Events>(type: Key, ...args: any) {
    if (this.events[type]) {
      this.events[type].forEach((fn) => fn.call(this, ...args));
    }
  }
  once<Key extends keyof Events>(type: Key, handler: Handler) {
    const inner = (...args: any) => {
      handler.call(this, ...args);
      this.off(type, inner);
    };
    this.on(type, inner);
  }
}

function test() {
  // demo
  let event = new EventEmitter();

  event.on('say', function (str) {
    console.log(str);
  });

  event.once('say', function (str) {
    console.log('这是once:' + str);
  });

  event.emit('say', '111');
  event.emit('say', '222');
  event.emit('say', '333');
}
