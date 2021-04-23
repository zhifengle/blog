export interface TNode<T> {
  val: T;
  left: null | TNode<T>;
  right: null | TNode<T>;
}

export class TreeNode<T> implements TNode<T> {
  val: T;
  left: null | TreeNode<T>;
  right: null | TreeNode<T>;
  constructor(
    val: T,
    left: TreeNode<T> | null = null,
    right: TreeNode<T> | null = null
  ) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}
