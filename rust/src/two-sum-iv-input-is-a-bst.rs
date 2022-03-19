// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use leetcode_prelude::TreeNode;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, set: &mut HashSet<i32>, k: i32) -> bool {
            if let Some(node) = node {
                if set.contains(&(k - node.as_ref().borrow().val)) {
                    return true;
                }
                set.insert(node.as_ref().borrow().val);
                dfs(node.as_ref().borrow().left.clone(), set, k)
                    || dfs(node.as_ref().borrow().right.clone(), set, k)
            } else {
                return false;
            }
        }
        dfs(root, &mut set, k)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use leetcode_prelude::btree;

    #[test]
    fn test() {
        assert_eq!(
            true,
            Solution::find_target(btree![5, 3, 6, 2, 4, null, 7], 9)
        );
        assert_eq!(
            false,
            Solution::find_target(btree![5, 3, 6, 2, 4, null, 7], 28)
        );
        assert_eq!(true, Solution::find_target(btree![2, 0, 3, -4, 1], -1));
    }
}
