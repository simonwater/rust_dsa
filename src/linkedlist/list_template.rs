use crate::linkedlist::ListNode;
/// []()
struct Solution;

impl Solution {
    pub fn main(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l1 = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let l2 = ListNode::from_vec(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::main(l1), l2);
    }
}
