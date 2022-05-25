use utils::Bisect;

struct Solution;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let n = nums.len();
        let is_odd = k & 1 == 1;
        let k2 = k >> 1;
        let mut window = vec![0; k];
        window.copy_from_slice(&nums[..k]);
        window.sort_unstable();

        let mut res = Vec::with_capacity(n + 1 - k);
        if is_odd {
            res.push(window[k2] as f64);
        } else {
            res.push((window[k2] as f64 + window[k2 - 1] as f64) / 2.0);
        }

        for i in k..n {
            if nums[i] > nums[i - k] {
                let hi = window.bisect_left(nums[i]);
                let lo = window[..hi].bisect_right(nums[i - k]);
                if hi > lo {
                    (&mut window[lo - 1..hi]).rotate_left(1);
                }
                window[hi - 1] = nums[i];
            } else if nums[i] < nums[i - k] {
                let hi = window.bisect_left(nums[i - k]);
                let lo = window[..hi].bisect_right(nums[i]);
                if hi > lo {
                    (&mut window[lo..hi + 1]).rotate_right(1);
                }
                window[lo] = nums[i];
            }
            if is_odd {
                res.push(window[k2] as f64);
            } else {
                res.push((window[k2] as f64 + window[k2 - 1] as f64) / 2.0);
            }
        }
        res
    }
}

#[test]
fn test_480() {
    assert_eq!(
        //vec![2.0, 3.0, 3.0, 2.0, 3.0, 2.0],
        vec![1.0, -1.0, -1.0, 3.0, 5.0, 6.0],
        Solution::median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
    );
    assert_eq!(
        vec![2.0, 3.0, 3.0, 3.0, 2.0, 3.0, 2.0],
        Solution::median_sliding_window(vec![1, 2, 3, 4, 2, 3, 1, 4, 2], 3)
    );

    assert_eq!(
        vec![2.5],
        Solution::median_sliding_window(vec![1, 4, 2, 3], 4)
    );
}
