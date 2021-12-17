//use std::collections::BinaryHeap;
use utils::Bisect;

struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix = vec![];
        prefix.push(vec![(i32::MIN, 1)]);
        let mut cur = 0;
        for i in 0..n {
            let mut j = cur;
            while nums[i] <= prefix[j][0].0 {
                j -= 1;
            }

            let mut count = 0;
            for &(v, c) in &prefix[j] {
                if nums[i] > v {
                    count += c;
                }
            }
            if j == cur {
                prefix.push(vec![(nums[i], count)]);
                cur += 1;
            } else {
                let ii = prefix[cur].bisect((nums[i], count));
                prefix[j + 1].insert(ii, (nums[i], count)); // same v, no add
            }
        }
        prefix[cur].iter().map(|(_, c)| c).sum()
    }
}

#[test]
fn test_673() {
    assert_eq!(2, Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]));
    assert_eq!(5, Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]));
    assert_eq!(
        2,
        Solution::find_number_of_lis(vec![-100, -99, -98, -97, -96, -96])
    );
}
