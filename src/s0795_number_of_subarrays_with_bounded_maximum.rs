struct Solution;

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut first = i32::MAX; // nums[first] <= right
        let mut last = i32::MIN;  // left <= nums[last] <= right

        let mut count = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num > right {
                first = i32::MAX;
                last = i32::MIN;
            } else if num < left {
                if last >= 0 {
                    count += 1 + last - first;
                } else {
                    first = first.min(i as i32);
                }
            } else {
                last = i as i32;
                first = first.min(i as i32);
                count += 1 + last - first;
            }
        }
        count
    }
}

#[test]
fn test_0795() {
    dbg!(Solution::num_subarray_bounded_max(
        vec![
            876, 880, 482, 260, 132, 421, 732, 703, 795, 420, 871, 445, 400, 291, 358, 589, 617,
            202, 755, 810, 227, 813, 549, 791, 418, 528, 835, 401, 526, 584, 873, 662, 13, 314,
            988, 101, 299, 816, 833, 224, 160, 852, 179, 769, 646, 558, 661, 808, 651, 982, 878,
            918, 406, 551, 467, 87, 139, 387, 16, 531, 307, 389, 939, 551, 613, 36, 528, 460, 404,
            314, 66, 111, 458, 531, 944, 461, 951, 419, 82, 896, 467, 353, 704, 905, 705, 760, 61,
            422, 395, 298, 127, 516, 153, 299, 801, 341, 668, 598, 98, 241
        ],
        658,
        719
    ));
}
