use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        let n = nums.len() as i32;
        let mut cache = HashSet::new();
        for i in 1..n / 2 + 1 {
            if sum * i % n == 0 {
                let target = sum * i / n;
                cache.clear();
                let mut stack = vec![(0, 0, 0)];
                while let Some((sum_now, count, cur)) = stack.pop() {
                    if sum_now == target && count == i {
                        return true;
                    }
                    if sum_now > target || count > i || cur == n {
                        continue;
                    }

                    for k in [
                        (sum_now, count, cur + 1),
                        (sum_now + nums[cur as usize], count + 1, cur + 1),
                    ] {
                        if cache.get(&k).is_none() {
                            stack.push(k);
                        }
                    }
                    cache.insert((sum_now, count, cur));
                }
            }
        }
        false
    }
}

#[test]
fn test_0805() {
    assert!(Solution::split_array_same_average(vec![
        1, 2, 3, 4, 5, 6, 7, 8
    ]));

    assert!(Solution::split_array_same_average(vec![5, 3, 11, 19, 2]));
    assert!(Solution::split_array_same_average(vec![5, 5]));
    dbg!(Solution::split_array_same_average(vec![
        60, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
        30, 30, 30, 30, 30, 30, 30
    ]));
}
