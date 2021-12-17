struct Solution;

const MAX_2: i32 = i32::MAX >> 1;

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut mc = MergeCount::new(nums);
        mc.count(0, mc.nums.len()) as i32
    }
}

struct MergeCount {
    nums: Vec<i32>,
    tmp: Vec<i32>,
}

impl MergeCount {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len() >> 1;
        Self {
            nums,
            tmp: vec![0; n],
        }
    }

    fn count(&mut self, lo: usize, hi: usize) -> usize {
        if lo + 1 == hi {
            0
        } else {
            let mid = lo + hi >> 1;
            let mut count = self.count(lo, mid) + self.count(mid, hi);

            let mut left = lo;
            for right in mid..hi {
                if self.nums[right] > MAX_2 {
                    break;
                } else if self.nums[right] < -MAX_2 {
                    count += mid - left;
                } else {
                    let v2 = self.nums[right] << 1;
                    while left < mid && self.nums[left] <= v2 {
                        left += 1;
                    }
                    count += mid - left;
                }
            }
            self.merge(lo, mid, hi);
            count
        }
    }

    fn merge(&mut self, mut lo: usize, mut mid: usize, hi: usize) {
        let hi2 = mid - lo;
        self.tmp[..hi2].clone_from_slice(&self.nums[lo..mid]);
        let mut lo2 = 0;
        loop {
            if self.tmp[lo2] > self.nums[mid] {
                self.nums[lo] = self.nums[mid];
                mid += 1;
                if mid == hi {
                    for i in lo2..hi2 {
                        lo += 1;
                        self.nums[lo] = self.tmp[i];
                    }
                    return;
                }
            } else {
                self.nums[lo] = self.tmp[lo2];
                lo2 += 1;
                if lo2 == hi2 {
                    return;
                }
            }
            lo += 1;
        }
    }
}

#[test]
fn test_493() {
    assert_eq!(2, Solution::reverse_pairs(vec![1, 3, 2, 3, 1]));
    assert_eq!(3, Solution::reverse_pairs(vec![2, 4, 3, 5, 1]));
    assert_eq!(1, Solution::reverse_pairs(vec![-5, -5]));
}
