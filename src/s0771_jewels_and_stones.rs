use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let jewels: HashSet<u8> = jewels.bytes().collect();
        stones.bytes().filter(|c| jewels.contains(&c)).count() as i32
    }
}

#[test]
fn test_771() {
    assert_eq!(
        3,
        Solution::num_jewels_in_stones("aA".to_owned(), "aAAbbbb".to_owned())
    );
    assert_eq!(
        0,
        Solution::num_jewels_in_stones("z".to_owned(), "ZZ".to_owned())
    );
}
