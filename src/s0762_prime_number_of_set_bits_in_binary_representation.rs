struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        Self::count_0(right) - Self::count_0(left - 1)
    }

    fn count_0(mut right: i32) -> i32 {
        let mut bc = [
            // binomial coefficient
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut res = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut count_bit = 0;
        while right > 0 {
            let mut i = count_bit;
            while i > 0 {
                bc[i] += bc[i - 1];
                i -= 1;
            }
            if (right & 1) == 1 {
                let mut i = count_bit + 1;
                while i > 0 {
                    res[i] = res[i - 1] + bc[i]; // use 1 + use 0
                    i -= 1;
                }
            }
            right >>= 1;
            count_bit += 1;
        }
        res[2] + res[3] + res[5] + res[7] + res[11] + res[13] + res[17] + res[19]
    }
}

#[test]
fn test_762() {
    assert_eq!(4, Solution::count_prime_set_bits(6, 10));
    assert_eq!(5, Solution::count_prime_set_bits(10, 15));
    assert_eq!(23, Solution::count_prime_set_bits(842, 888));

    assert_eq!(0, Solution::count_0(0));
    assert_eq!(454, Solution::count_0(841));
    assert_eq!(19, Solution::count_0(26));
    assert_eq!(17, Solution::count_0(24));
    dbg!(Solution::count_0(1000000));
}
