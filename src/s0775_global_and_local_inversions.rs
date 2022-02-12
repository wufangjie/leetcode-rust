struct Solution;

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        for (i, v) in nums.into_iter().enumerate() {
            let i = i as i32;
            if (v > i + 1) || (i > v + 1) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_775() {
    assert!(Solution::is_ideal_permutation(vec![1, 0, 2]));
    assert!(!Solution::is_ideal_permutation(vec![1, 2, 0]));
}
