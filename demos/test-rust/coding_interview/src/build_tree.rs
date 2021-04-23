// use std::cell::RefCell;
// use std::rc::Rc;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

use crate::base_st::TreeNode;

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.len() == 0 {
        return None;
    }
    let n = preorder.len();
    let mut mp: HashMap<i32, i32> = HashMap::new();
    for i in 0..n {
        // 注意是中序的 HashMap
        mp.insert(inorder[i], i as i32);
    }
    fn recur(
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
        pre_left: i32,
        pre_right: i32,
        in_left: i32,
        in_right: i32,
        mp: &HashMap<i32, i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_left > pre_right {
            return None;
        }
        let pre_root_idx = pre_left;
        let in_root_idx = *mp.get(&preorder[pre_root_idx as usize]).unwrap();

        let left_size = in_root_idx - in_left;
        let root_val = preorder[pre_root_idx as usize];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        root.borrow_mut().left = recur(
            preorder,
            inorder,
            pre_left + 1,
            pre_left + left_size,
            in_left,
            in_root_idx - 1,
            mp,
        );
        root.borrow_mut().right = recur(
            preorder,
            inorder,
            pre_left + left_size + 1,
            pre_right,
            in_root_idx + 1,
            in_right,
            mp,
        );
        Some(root)
    }

    recur(
        &preorder,
        &inorder,
        0,
        (n - 1) as i32,
        0,
        (n - 1) as i32,
        &mp,
    )
}

// 这个是别人的写法
pub fn build_tree2(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    helper(&preorder, &inorder)
    // 在 Solution 里面可以这么写
    // Self::helper(&preorder, &inorder)
}

fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(root) = preorder.first() {
        let pivot_idx = inorder
            .iter()
            .enumerate()
            .find(|(_, v)| v == &root)
            .unwrap()
            .0;
        return Some(Rc::new(RefCell::new(TreeNode {
            val: *root,
            left: helper(&preorder[1..(1 + pivot_idx)], &inorder[0..pivot_idx]),
            right: helper(&preorder[(1 + pivot_idx)..], &inorder[(pivot_idx + 1)..]),
        })));
    } else {
        return None;
    }
}
