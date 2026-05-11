use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// []()
struct Solution;

impl Solution {
    pub fn main(root1: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let tree1 = tree![1, 3, 2, 5];
        let tree2 = tree![1, 3, 2, 5];
        assert_eq!(Solution::main(tree1), tree2);
    }
}
