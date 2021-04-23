import { TreeNode } from '../TreeNode';

// 递归的方法
// 先序遍历:  [根, [左], [右]]
// 中序遍历:  [[left], root, [right]]
export function buildTree(
  preorder: number[],
  inorder: number[]
): TreeNode<number> {
  const n = preorder.length;
  const indexMap = new Map<number, number>();

  for (let i = 0; i < n; i++) {
    indexMap.set(inorder[i], i);
  }
  function myBuildTree(
    preorder: number[],
    inorder: number[],
    preLeft: number,
    preRight: number,
    inLeft: number,
    inRight: number
  ): TreeNode<number> {
    if (preLeft > preRight) return null;

    // pre root
    let preRootIdx = preLeft;

    const inRootIdx = indexMap.get(preorder[preRootIdx]);

    const root: TreeNode<number> = new TreeNode(preorder[preRootIdx]);

    const leftSize = inRootIdx - inLeft;
    root.left = myBuildTree(
      preorder,
      inorder,
      preLeft + 1,
      preLeft + leftSize,
      inLeft,
      inRootIdx - 1
    );
    root.right = myBuildTree(
      preorder,
      inorder,
      preLeft + 1 + leftSize,
      preRight,
      inRootIdx + 1,
      inRight
    );
    return root;
  }
  return myBuildTree(preorder, inorder, 0, n - 1, 0, n - 1);
}
// 递归法
// Rust 可以使用这种方式
// 使用一个 Data 结构体存储  p 和 d. 或者将 p ,d 当作参数进行递归调用
export function buildTree4(preorder: number[], inorder: number[]) {
  const p: IterableIterator<number> = preorder[Symbol.iterator]();
  const d: Map<number, number> = new Map(inorder.map((v, k) => [v, k]));
  const helper = (i: number, j: number): TreeNode<number> | null => {
    if (i < j) {
      const val = p.next().value,
        idx = <number>d.get(val);
      return new TreeNode(val, helper(i, idx), helper(idx + 1, j));
    }
    return null;
  };
  return helper(0, inorder.length);
}

// 迭代法
export function buildTree2(
  preorder: number[],
  inorder: number[]
): TreeNode<number> {
  if (!preorder || preorder.length === 0) {
    return null;
  }
  const root = new TreeNode(preorder[0]);

  const stack: TreeNode<number>[] = [];
  stack.push(root);
  // 中序索引
  let inorderIndex = 0;
  // 遍历先序
  for (let i = 1; i < preorder.length; i++) {
    const val = preorder[i];
    let node = stack[stack.length - 1];

    if (node.val !== inorder[inorderIndex]) {
      node.left = new TreeNode(val);
      stack.push(node.left);
    } else {
      while (
        !!stack.length &&
        stack[stack.length - 1].val === inorder[inorderIndex]
      ) {
        node = stack.pop();
        inorderIndex++;
      }
      node.right = new TreeNode(val);
      stack.push(node.right);
    }
  }
  return root;
}

// 速度偏慢. Rust 可以使用这种递归的方法
export function buildTree3(preorder: number[], inorder: number[]) {
  function helper(preorder: number[], inorder: number[]): TreeNode<number> {
    if (preorder && preorder.length) {
      const first = preorder[0];
      const pivotIdx = inorder.findIndex((x) => x === first);
      const node = new TreeNode(first);
      node.left = helper(
        preorder.slice(1, 1 + pivotIdx),
        inorder.slice(0, pivotIdx)
      );
      node.right = helper(
        preorder.slice(1 + pivotIdx),
        inorder.slice(1 + pivotIdx)
      );
      return node;
    } else {
      return null;
    }
  }
  return helper(preorder, inorder);
}
