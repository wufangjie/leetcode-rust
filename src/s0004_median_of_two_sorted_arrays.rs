struct Solution;

impl Solution {
    /// O(m+n) rather than O(log(m+n))
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();

        let skip = (m + n - 1) >> 1; // m + n >= 1
        let is_odd = skip * 2 + 1 == m + n;

        nums1.push(i32::MAX);
        nums2.push(i32::MAX);

        let mut x = nums1[0];
        let mut y = nums2[0];

        let mut i = 0;
        let mut j = 0;

        for _ in 0..skip {
            if x < y {
                i += 1;
                x = nums1[i];
            } else {
                j += 1;
                y = nums2[j];
            }
        }

        if is_odd {
            x.min(y) as f64
        } else {
            (if x < y {
                i += 1;
                x + y.min(nums1[i])
            } else {
                j += 1;
                y + x.min(nums2[j])
            }) as f64
                / 2.0
        }
    }
}

// impl Solution {
//     /// 太臃肿，且容易出错
//     pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
//         let m = nums1.len();
//         let n = nums2.len();

//         let mut skip = (m + n - 1) >> 1; // m + n >= 1
//         let is_odd = skip * 2 + 1 == m + n;

//         let mut i = 0;
//         let mut j = 0;

//         loop {
//             if i == m {
//                 if is_odd {
//                     return nums2[j + skip] as f64;
//                 } else {
//                     return (nums2[j + skip] + nums2[j + skip + 1]) as f64 / 2.0;
//                 }
//             } else if j == n {
//                 if is_odd {
//                     return nums1[i + skip] as f64;
//                 } else {
//                     return (nums1[i + skip] + nums1[i + skip + 1]) as f64 / 2.0;
//                 }
//             } else {
//                 if skip == 0 {
//                     if is_odd {
//                         return nums1[i].min(nums2[j]) as f64;
//                     } else if nums1[i] < nums2[j] {
//                         if i + 1 == m || nums2[j] < nums1[i + 1] {
//                             return (nums1[i] + nums2[j]) as f64 / 2.0;
//                         } else {
//                             return (nums1[i] + nums1[i + 1]) as f64 / 2.0;
//                         }
//                     } else {
//                         if j + 1 == n || nums1[i] < nums2[j + 1] {
//                             return (nums1[i] + nums2[j]) as f64 / 2.0;
//                         } else {
//                             return (nums2[j] + nums2[j + 1]) as f64 / 2.0;
//                         }
//                     }
//                 }
//                 skip -= 1;
//                 if nums1[i] < nums2[j] {
//                     i += 1;
//                 } else {
//                     j += 1;
//                 }
//             }
//         }
//     }
// }

#[test]
fn test_0004() {
    assert_eq!(
        2.0,
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
    );
    assert_eq!(
        2.5,
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
    );
    assert_eq!(2.0, Solution::find_median_sorted_arrays(vec![], vec![2]));

    assert_eq!(
        0.0,
        Solution::find_median_sorted_arrays(vec![0, 0, 0, 0, 0], vec![-1, 0, 0, 0, 0, 0, 1])
    );
}
