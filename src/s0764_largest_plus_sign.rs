use std::collections::BinaryHeap;
// use utils::Heap;

struct Solution;

impl Solution {
    fn find_dv(col: &Vec<i32>, i: i32, n: i32) -> i32 {
        let mut lo = 0;
        let mut hi = col.len() as i32 - 1;
        while hi >= lo {
            let mid = (lo + hi) >> 1;
            if col[mid as usize] > i {
                hi = mid - 1; // usize will stackoverflow
            } else {
                lo = mid + 1; // no == condition
            }
        }

        let lo_usize = lo as usize;
        let d_up = if lo == 0 {
            i + 1
        } else {
            i - col[lo_usize - 1]
        };
        let d_down = if lo_usize == col.len() {
            n - i
        } else {
            col[lo_usize] - i
        };
        d_up.min(d_down)
    }

    pub fn order_of_largest_plus_sign(n: i32, mut mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        if mines.len() == n.pow(2) {
            return 0;
        }

        mines.sort_by(|a, b| (1000 * a[0] + a[1]).cmp(&(1000 * b[0] + b[1])));
        let mut rows = vec![vec![]; n];
        for pair in &mines {
            rows[pair[0] as usize].push(pair[1]);
        }
        mines.sort_by(|a, b| (1000 * a[1] + a[0]).cmp(&(1000 * b[1] + b[0])));
        let mut cols = vec![vec![]; n];
        for pair in &mines {
            cols[pair[1] as usize].push(pair[0]);
        }

        let mut heap = BinaryHeap::new();
        //let mut heap = Heap::new();
        for i in 1..n - 1 {
            let dv = (i + 1).min(n - i) as i32;
            let mut pre = -1;
            for j in &rows[i] {
                let count = j - pre - 1;
                if count >= 3 {
                    let d4 = dv.min((count + 1) >> 1);
                    heap.push((d4, i, pre + 1, count))
                }
                pre = *j;
            }
            let count = n as i32 - pre - 1;
            if count >= 3 {
                let d4 = dv.min((count + 1) >> 1);
                heap.push((d4, i, pre + 1, count))
            }
        }

        let mut ret = 1;
        while let Some((d4, i, lo, count)) = heap.pop() {
            if d4 <= ret {
                return ret;
            }
            let mut j = ret;
            while j < count - ret {
                let dh = (j + 1).min(count - j);
                if dh <= ret {
                    continue;
                }
                let dv = Self::find_dv(&cols[(lo + j) as usize], i as i32, n as i32);
                ret = ret.max(dv.min(dh));
                j += 1;
            }
        }
        ret
    }
}

#[test]
fn test_764() {
    assert_eq!(2, Solution::order_of_largest_plus_sign(5, vec![vec![4, 2]]));
    assert_eq!(0, Solution::order_of_largest_plus_sign(1, vec![vec![0, 0]]));
    assert_eq!(2, Solution::order_of_largest_plus_sign(3, vec![vec![0, 0]]));
    assert_eq!(1, Solution::order_of_largest_plus_sign(3, vec![vec![0, 1]]));
}
