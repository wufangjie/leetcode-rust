use std::collections::VecDeque;

struct Solution;

impl Solution {
    /// short solution
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut deque = VecDeque::with_capacity(k + 1);
        let mut res = Vec::with_capacity(n - k + 1);
        for (i, &v) in nums.iter().enumerate() {
            while let Some(&p) = deque.back() {
		// learned usage: back() // last
                if p < v {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(v);
            if i >= k {
                if deque[0] == nums[i - k] {
                    deque.pop_front();
                }
                res.push(deque[0]);
            } else if i >= k - 1 {
                res.push(deque[0]);
            }
        }
        res
    }
}

impl Solution {
    /// faster solution (into_iter() ?)
    pub fn max_sliding_window_faster(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut deque = VecDeque::with_capacity(k + 1);
        deque.push_back((nums[0], 0));
        let mut nums = nums.into_iter();
        let mut si = 1;
        for (i, v) in nums.by_ref().take(k).enumerate().skip(1) {
            while si > 0 {
                if deque[si - 1].0 <= v {
                    deque.pop_back();
                    si -= 1;
                } else {
                    break;
                }
            }
            deque.push_back((v, i));
            si += 1;
        }

        let mut res = Vec::with_capacity(n - k + 1);
        res.push(deque[0].0);

        let mut i = k;
        for v in nums {
            while si > 0 {
                if deque[si - 1].0 <= v {
                    deque.pop_back();
                    si -= 1;
                } else {
                    break;
                }
            }
            deque.push_back((v, i));
            si += 1;
            if deque[0].1 <= i - k {
                deque.pop_front();
                si -= 1;
            }
            i += 1;
            res.push(deque[0].0)
        }
        res
    }
}

#[test]
fn test_239() {
    assert_eq!(
        vec![3, 3, 5, 5, 6, 7],
        Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
    );
    assert_eq!(vec![1], Solution::max_sliding_window(vec![1], 1));
    assert_eq!(vec![1, -1], Solution::max_sliding_window(vec![1, -1], 1));

    assert_eq!(
        vec![3, 3, 5, 5, 6, 7],
        Solution::max_sliding_window_faster(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
    );
    assert_eq!(vec![1], Solution::max_sliding_window_faster(vec![1], 1));
    assert_eq!(
        vec![1, -1],
        Solution::max_sliding_window_faster(vec![1, -1], 1)
    );
}
