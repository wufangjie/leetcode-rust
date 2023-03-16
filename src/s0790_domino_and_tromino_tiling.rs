struct Solution;

const MODULO: i64 = 10i64.pow(9) + 7;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        if n < 3 {
            return n;
        }
        let n = n as usize;
        let mut dp0: Vec<i64> = vec![0; n + 1]; // full
        let mut dp1: Vec<i64> = vec![0; n];     // pad1
        dp0[0] = 1;
        dp0[1] = 1;
        dp0[2] = 2;
        for i in 3..=n {
            dp1[i - 2] = (dp0[i - 3] * 2 + dp1[i - 3]) % MODULO;
            dp0[i] = (dp0[i - 2] + dp0[i - 1] + dp1[i - 2]) % MODULO; // case =, |, 7
        }
        dp0[n] as i32
    }
}

#[test]
fn test_0790() {
    assert_eq!(5, Solution::num_tilings(3));
    assert_eq!(312342182, Solution::num_tilings(30));
}
