import { ListNode } from '../links';

// 06
export const reversePrint = (head: ListNode<number>) => {
  let res: number[] = [];

  while (head) {
    res.push(head.val);
    head = head.next;
  }
  res.reverse();
  return res;
};
// 05
export function replaceSpace(s: string): string {
  //   return s.replace(' ', '%20');
  let newStr = '';
  for (let i = 0; i < s.length; i++) {
    if (s[i] == ' ') {
      newStr += '%20';
    } else {
      newStr += s[i];
    }
  }

  return newStr;
}
// 03
export function findRepeatNumber(nums: number[]): number {
  let mp = new Map();
  for (const num of nums) {
    if (mp.get(num) !== undefined) {
      return num;
    } else {
      mp.set(num, num);
    }
  }
  return -1;
}
