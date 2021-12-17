// use std::collections::{HashMap, HashSet};
// use utils::Bisect;
// use utils::SegmentTree;

struct Solution;

impl Solution {
    // pub fn count_smaller_bisect(nums: Vec<i32>) -> Vec<i32> {
    //     // 576ms
    //     let n = nums.len();
    //     let mut tail = Vec::with_capacity(n);
    //     let mut res = vec![0; n];
    //     for i in 1..=n {
    //         let ii = n - i;
    //         let j = tail.bisect_left(nums[ii]);
    //         res[ii] = j as i32;
    //         tail.insert(j, nums[ii]);
    //     }
    //     res
    // }

    // pub fn count_smaller_segment_tree(nums: Vec<i32>) -> Vec<i32> {
    //     // 204ms
    //     let n = nums.len();
    //     let dct: HashSet<i32> = nums.iter().cloned().collect();
    //     let mut lst: Vec<i32> = dct.into_iter().collect();
    //     lst.sort_unstable();
    //     let m = lst.len();
    //     let dct: HashMap<i32, usize> = lst.into_iter().zip(0..m).collect();

    //     let mut st = SegmentTree::empty(m);
    //     let mut res = vec![0; n];
    //     for i in 1..=n {
    //         let ii = n - i;
    //         let j = *dct.get(&nums[ii]).unwrap();
    //         st.update_by_diff(j, 1);
    //         if j > 0 {
    //             res[ii] = st.query(0, j - 1);
    //         }
    //     }
    //     res
    // }

    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        // 88ms
        let mut mc = MergeCount::new(nums);
        mc.sort(0, mc.nums.len());
        mc.count
    }
}

struct MergeCount {
    nums: Vec<i32>,
    mask: Vec<usize>,
    count: Vec<i32>,
    tmp: Vec<usize>, // 150ms -> 88ms
}

impl MergeCount {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        Self {
            nums,
            mask: (0..n).into_iter().collect(),
            count: vec![0; n],
            tmp: vec![0; n >> 1],
        }
    }

    fn sort(&mut self, mut lo: usize, hi: usize) {
        if lo + 1 < hi {
            let mut mid = lo + hi >> 1;
            self.sort(lo, mid);
            self.sort(mid, hi);
            let mut delta = 0;
            let hi2 = mid - lo;
            self.tmp[..hi2].copy_from_slice(&self.mask[lo..mid]);
            let mut lo2 = 0;
            loop {
                if self.nums[self.tmp[lo2]] > self.nums[self.mask[mid]] {
                    self.mask[lo] = self.mask[mid];
                    mid += 1;
                    delta += 1;
                    if mid == hi {
                        for i in lo2..hi2 {
                            lo += 1;
                            self.mask[lo] = self.tmp[i];
                            self.count[self.tmp[i]] += delta;
                        }
                        return;
                    }
                } else {
                    self.mask[lo] = self.tmp[lo2];
                    self.count[self.tmp[lo2]] += delta;
                    lo2 += 1;
                    if lo2 == hi2 {
                        return;
                    }
                }
                lo += 1;
            }
        }
    }
}

#[test]
fn test_315() {
    assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
}
