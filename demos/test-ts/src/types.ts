export type BPropType = string | number | symbol;

export type Func = (...args: any) => any;

export interface UnaryFunc<T, R> {
  (source: T): R;
}

// https://www.typescriptlang.org/docs/handbook/utility-types.html
// 取交集
// type SomeTest = Extract<BPropType, string | number>

// Pick<Todo, 'completed' | 'xxx'>
// Omit<Type, Keys>

// type T1 = ReturnType<(s: string) => void>;
