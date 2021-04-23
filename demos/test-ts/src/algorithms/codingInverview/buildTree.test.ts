import { TreeNode } from '../TreeNode';
import { buildTree, buildTree2, buildTree3 } from './buildTree';

function isEqualTree<T>(tree1: TreeNode<T>, tree2: TreeNode<T>): boolean {
  if (tree1 && !tree2) return false;
  if (!tree1 && tree2) return false;
  if (!tree1 && !tree2) return true;

  if (tree1.val !== tree2.val) {
    return false;
  } else {
    return (
      isEqualTree(tree1.left, tree2.left) &&
      isEqualTree(tree1.right, tree2.right)
    );
  }
}
test('build tree', () => {
  const originTree = new TreeNode(3);
  originTree.left = new TreeNode(9);
  originTree.right = new TreeNode(20);
  originTree.right.left = new TreeNode(15);
  originTree.right.right = new TreeNode(7);

  expect(
    isEqualTree(buildTree([3, 9, 20, 15, 7], [9, 3, 15, 20, 7]), originTree)
  ).toEqual(true);
  expect(
    isEqualTree(buildTree2([3, 9, 20, 15, 7], [9, 3, 15, 20, 7]), originTree)
  ).toEqual(true);
  expect(
    isEqualTree(buildTree3([3, 9, 20, 15, 7], [9, 3, 15, 20, 7]), originTree)
  ).toEqual(true);
});
