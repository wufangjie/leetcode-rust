struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0i32;
        let mut hi = nums.len() as i32 - 1;
        while lo <= hi {
            let mid = (lo + hi) >> 1;
            match nums[mid as usize] {
                n if n == target => return mid,
                n if n < target => lo = mid + 1,
                _ => hi = mid - 1,
            }
        }
        -1
    }
}

#[test]
fn test_704() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
}
