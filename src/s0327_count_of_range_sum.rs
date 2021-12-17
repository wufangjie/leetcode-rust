//use utils::Bisect;

struct Solution;

impl Solution {
    // pub fn count_range_sum_bisect(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
    //     // 1561ms
    //     let mut count = 0;
    //     let mut acc = 0;
    //     let lower = lower as i64;
    //     let upper = upper as i64;
    //     let mut pre: Vec<i64> = Vec::with_capacity(nums.len());
    //     for elem in nums {
    //         let elem = elem as i64;
    //         let i = pre.bisect_left(lower - acc - elem);
    //         let j = pre.bisect_right(upper - acc - elem);
    //         count += j - i;
    //         if elem >= lower && elem <= upper {
    //             count += 1;
    //         }
    //         pre.insert(pre.bisect(-acc), -acc);
    //         acc += elem;
    //     }
    //     count as i32
    // }

    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        // 88ms
        let n = nums.len();
        let mut cumsum = vec![0; n];
        cumsum[0] = nums[0] as i64;
        for i in 1..n {
            cumsum[i] = cumsum[i - 1] + nums[i] as i64;
        }
        let mut mc = MergeCount::new(cumsum, lower as i64, upper as i64);
        mc.count(0, n) as i32
    }
}

struct MergeCount {
    cumsum: Vec<i64>,
    lower: i64,
    upper: i64,
    tmp: Vec<i64>,
}

impl MergeCount {
    fn new(cumsum: Vec<i64>, lower: i64, upper: i64) -> Self {
        let n = cumsum.len() >> 1;
        Self {
            cumsum,
            lower,
            upper,
            tmp: vec![0; n],
        }
    }

    fn count(&mut self, lo: usize, hi: usize) -> usize {
        if lo + 1 == hi {
            (self.cumsum[lo] >= self.lower && self.cumsum[lo] <= self.upper) as usize
        } else {
            let mid = lo + hi >> 1;
            let mut count = self.count(lo, mid) + self.count(mid, hi);

            let mut rlo = mid;
            let mut rhi = mid;
            for left in lo..mid {
                while rlo < hi && self.cumsum[rlo] - self.cumsum[left] < self.lower {
                    rlo += 1;
                }
                rhi = rhi.max(rlo);
                while rhi < hi && self.cumsum[rhi] - self.cumsum[left] <= self.upper {
                    rhi += 1;
                }
                count += rhi - rlo;
            }
            self.merge(lo, mid, hi);
            count
        }
    }

    #[inline]
    fn merge(&mut self, mut lo: usize, mut mid: usize, hi: usize) {
        if self.cumsum[mid] >= self.cumsum[mid - 1] {
            return;
        }
        let hi2 = mid - lo;
        self.tmp[..hi2].copy_from_slice(&self.cumsum[lo..mid]);
        let mut lo2 = 0;
        loop {
            if self.tmp[lo2] > self.cumsum[mid] {
                self.cumsum[lo] = self.cumsum[mid];
                mid += 1;
                if mid == hi {
                    for i in lo2..hi2 {
                        lo += 1;
                        self.cumsum[lo] = self.tmp[i];
                    }
                    break;
                }
            } else {
                self.cumsum[lo] = self.tmp[lo2];
                lo2 += 1;
                if lo2 == hi2 {
                    break;
                }
            }
            lo += 1;
        }
    }
}

#[test]
fn test_327() {
    assert_eq!(3, Solution::count_range_sum(vec![-2, 5, -1], -2, 2));
    assert_eq!(
        3,
        Solution::count_range_sum(vec![-2147483647, 0, -2147483647, 2147483647], -564, 3864)
    );
    assert_eq!(3, Solution::count_range_sum(vec![0, 0], 0, 0));
}
