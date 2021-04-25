// 06
import { INode } from '../links';
import { replaceSpace, reversePrint } from '.';
import { fib, fib2 } from './fib';

test('reverse print', () => {
  let testHead: INode<number> = {
    val: 1,
    next: {
      val: 2,
      next: {
        val: 3,
        next: {
          val: 4,
          next: {
            val: 5,
            next: null,
          },
        },
      },
    },
  };
  expect(reversePrint(testHead)).toEqual([5, 4, 3, 2, 1]);
});

test('replace space', () => {
  const s = 'We are happy.';
  expect(replaceSpace(s)).toEqual('We%20are%20happy.');
});

test('fib', () => {
  expect(fib(45)).toEqual(134903163);
  expect(fib2(10)).toEqual(55);
});
