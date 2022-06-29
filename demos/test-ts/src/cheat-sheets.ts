// https://www.typescriptlang.org/cheatsheets

const data2 = {
  name: 'Foo',
} as const;

// type data2 = {
//   name: 'Foo';
// };

type Responses =
  | { status: 200; data: any }
  | { status: 301; to: string }
  | { status: 400; error: Error };

type Arrayish = { [n: number]: unknown };
type A = keyof Arrayish;

function f() {
  return { x: 10, y: 3 };
}
type P = ReturnType<typeof f>;
