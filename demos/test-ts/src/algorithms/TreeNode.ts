export interface TNode<T> {
  val: T;
  left: null | TNode<T>;
  right: null | TNode<T>;
}

export class TreeNode<T> implements TNode<T> {
  val: T;
  left: null | TreeNode<T>;
  right: null | TreeNode<T>;
  constructor(val: T) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}
