export interface INode<T> {
  val: T;
  next: null | INode<T>;
}

export class ListNode<T> implements INode<T> {
  val: T;
  next: null | ListNode<T>;
  constructor(val: T) {
    this.val = val;
    this.next = null;
  }
}

export const reverseLinkedList = function <T>(head: INode<T>) {
  let h: null | INode<T> = null;
  let p: null | INode<T> = null;
  while (head) {
    p = head.next;
    head.next = h;
    h = head;
    head = p;
  }
  return h;
};

const testHead: INode<number> = {
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
