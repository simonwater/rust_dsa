/// [409. 最长回文串](https://leetcode.cn/problems/longest-palindrome/description/)
struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut cnts = [0; 60];
        let s = s.as_bytes();
        for &c in s {
            let idx = (c - b'A') as usize;
            cnts[idx] += 1;
        }
        let mut odd_cnt = 0;
        for cnt in cnts {
            if cnt % 2 == 1 {
                odd_cnt += 1;
            }
        }
        let len = s.len() as i32;
        if odd_cnt > 0 { len - odd_cnt + 1 } else { len }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("abccccdd");
        let ans = 7;
        assert_eq!(Solution::longest_palindrome(s), ans);

        let s = String::from("a");
        let ans = 1;
        assert_eq!(Solution::longest_palindrome(s), ans);

        let s = String::from("aa");
        let ans = 2;
        assert_eq!(Solution::longest_palindrome(s), ans);
    }
}
