use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        if vals.len() == 0 || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut q = VecDeque::new();
        q.push_back(Rc::clone(&root));
        let mut i = 1;
        while i < vals.len() {
            if let Some(parent) = q.pop_front() {
                if let Some(val) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    q.push_back(Rc::clone(&left));
                    parent.borrow_mut().left = Some(left);
                }
                i += 1;
                if i < vals.len() {
                    if let Some(val) = vals[i] {
                        let right = Rc::new(RefCell::new(TreeNode::new(val)));
                        q.push_back(Rc::clone(&right));
                        parent.borrow_mut().right = Some(right);
                    }
                }
                i += 1;
            }
        }
        Some(root)
    }
}

// --- 宏定义 ---
#[macro_export]
macro_rules! tree {
    // 匹配空
    () => { None };
    // 匹配列表并转换成构造函数能接受的格式
    ($($val:tt),*) => {
        {
            let mut v = Vec::new();
            $(
                // 将 null 转换为 None，数字转换为 Some
                v.push(tree!(@inner $val));
            )*
            TreeNode::from_vec(v)
        }
    };
    // 内部辅助：处理 null 关键字
    (@inner null) => { None };
    // 内部辅助：处理表达式（数字）
    (@inner $e:expr) => { Some($e) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(tree![1, 3, 2, 5], tree![1, 3, 2, 5]);
        assert_eq!(
            tree![2, 1, 3, null, 4, null, 7],
            tree![2, 1, 3, null, 4, null, 7]
        );
    }
}
