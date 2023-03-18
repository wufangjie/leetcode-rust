use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        let mut hi = k + 1;
        let mut lo = 0;
        while lo < hi {
            let mid = (hi + lo) >> 1;
            match Self::guess(mid).cmp(&k) {
                Ordering::Equal => return 5,
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid,
            }
        }
        0
    }

    #[inline]
    fn guess(mut k: i32) -> i32 {
        let mut res = k;
        while k > 4 {
            k /= 5;
            res += k;
        }
        res
    }
}

#[test]
fn test_0793() {
    assert_eq!(5, Solution::preimage_size_fzf(1000000000));
}

// /// actually it's k*5's count fractor of 5
// #[inline]
// fn count_fractor_5(mut k: i32) -> i32 {
//     let mut res = k;
//     while k % 5 == 0 {
//         res += 1;
//         k /= 5;
//     }
//     return res;
// }
