struct Solution;

impl Solution {
    pub fn kth_grammar(n: i32, mut k: i32) -> i32 {
        let mut mid = 2i32.pow(n as u32 - 1);
        k -= 1;
        let mut res = 0;
        for _ in 1..n {
            mid >>= 1;
            if k >= mid {
                res ^= 1;
                k -= mid;
            }
        }
        res
    }
}

#[test]
fn test_779() {
    assert_eq!(0, Solution::kth_grammar(1, 1));
    assert_eq!(0, Solution::kth_grammar(2, 1));
    assert_eq!(1, Solution::kth_grammar(2, 2));
    for (i, c) in "0110100110010110".chars().enumerate() {
        assert_eq!(c as u8 - 48, Solution::kth_grammar(5, 1 + i as i32) as u8);
    }
}

// 0
// 01
// 0110
// 01101001
// 0110100110010110
