use crate::linkedlist::ListNode;
/// [206. 反转链表](https://leetcode.cn/problems/reverse-linked-list/description/)
struct Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::recursive(None, head)
    }

    fn recursive(prev: Option<Box<ListNode>>, cur: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match cur {
            None => prev,
            Some(mut node) => {
                let next = node.next.take();
                node.next = prev;
                Self::recursive(Some(node), next)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let ll = ListNode::from_vec(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list(l), ll);

        let l = ListNode::from_vec(vec![1, 2]);
        let ll = ListNode::from_vec(vec![2, 1]);
        assert_eq!(Solution::reverse_list(l), ll);

        let l = ListNode::from_vec(vec![]);
        let ll = ListNode::from_vec(vec![]);
        assert_eq!(Solution::reverse_list(l), ll);
    }

    #[test]
    fn test2() {
        let l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let ll = ListNode::from_vec(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list_recursive(l), ll);

        let l = ListNode::from_vec(vec![1, 2]);
        let ll = ListNode::from_vec(vec![2, 1]);
        assert_eq!(Solution::reverse_list_recursive(l), ll);

        let l = ListNode::from_vec(vec![]);
        let ll = ListNode::from_vec(vec![]);
        assert_eq!(Solution::reverse_list_recursive(l), ll);
    }
}
