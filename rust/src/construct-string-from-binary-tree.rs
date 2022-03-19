use std::cell::RefCell;
use std::rc::Rc;
use leetcode_prelude::{TreeNode};

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> String {
            if let Some(root) = root {
                let root = root.borrow();
                let val = root.val.to_string();
                return match (root.left.clone(), root.right.clone()) {
                    (None, None) => val,
                    (_, None) => format!("{}({})", val, dfs(root.left.clone())),
                    (_, _) => format!("{}({})({})", val, dfs(root.left.clone()), dfs(root.right.clone()))
                }
            }
            "".to_string()
        }
        dfs(root)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use leetcode_prelude::btree;

    #[test]
    fn test() {
        assert_eq!("1(2()(4))(3)", Solution::tree2str(btree![1,2,3,null,4]));
    }
}
