use std::collections::HashMap;

/// [1. 两数之和](https://leetcode.cn/problems/two-sum/description/)
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&before) = map.get(&(target - num)) {
                return vec![before as i32, i as i32];
            }
            map.insert(num, i);
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 7, 11, 15];
        let ans = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, 9), ans);

        let nums = vec![3, 2, 4];
        let ans = vec![1, 2];
        assert_eq!(Solution::two_sum(nums, 6), ans);

        let nums = vec![3, 3];
        let ans = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, 6), ans);
    }
}
