use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [863. 二叉树中所有距离为 K 的结点](https://leetcode.cn/problems/all-nodes-distance-k-in-binary-tree/)
struct Solution;

use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        if root.is_none() || target.is_none() {
            return Vec::new();
        }
        let mut parents: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::with_capacity(16);
        Self::init_parents(&root, &None, &mut parents);

        let mut visited = [false; 501];
        let mut q = VecDeque::new();
        let node = target.as_ref().unwrap();
        visited[node.borrow().val as usize] = true;
        q.push_back((Rc::clone(node), 0));

        let mut ans: Vec<i32> = Vec::with_capacity(16);
        while let Some((node_rc, dist)) = q.pop_front() {
            let node = node_rc.borrow();
            if dist == k {
                ans.push(node.val);
            } else {
                if let Some(left_rc) = &node.left {
                    let left_val = left_rc.borrow().val;
                    if !visited[left_val as usize] {
                        visited[left_val as usize] = true;
                        q.push_back((Rc::clone(left_rc), dist + 1));
                    }
                }
                if let Some(right_rc) = &node.right {
                    let right_val = right_rc.borrow().val;
                    if !visited[right_val as usize] {
                        visited[right_val as usize] = true;
                        q.push_back((Rc::clone(right_rc), dist + 1));
                    }
                }
                if let Some(parent_rc) = parents.get(&node.val) {
                    let parent_val = parent_rc.borrow().val;
                    if !visited[parent_val as usize] {
                        visited[parent_val as usize] = true;
                        q.push_back((Rc::clone(parent_rc), dist + 1));
                    }
                }
            }
        }

        ans
    }

    fn init_parents(
        root: &Option<Rc<RefCell<TreeNode>>>,
        parent: &Option<Rc<RefCell<TreeNode>>>,
        parents: &mut HashMap<i32, Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            if let Some(parent_rc) = parent {
                parents.insert(node.val, Rc::clone(parent_rc));
            }

            Self::init_parents(&node.left, root, parents);
            Self::init_parents(&node.right, root, parents);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {}
}
