struct Solution;

impl Solution {
    pub fn main(s1: String, s2: String) -> usize {
        s1.len() + s2.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let text1 = String::from("value1");
        let text2 = String::from("value2");
        assert_eq!(Solution::main(text1, text2), 0);
    }

    #[test]
    fn test2() {}
}
