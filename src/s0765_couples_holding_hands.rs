use utils::Heap;

struct Solution;

impl Solution {
    #[inline]
    fn is_couple(a: i32, b: i32) -> bool {
        if a > b {
            a - b == a & 1
        } else {
            b - a == b & 1 // a != b
        }
    }

    #[inline]
    fn find_half(a: i32) -> i32 {
        if a & 1 == 1 {
            a - 1
        } else {
            a + 1
        }
    }

    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let mut heap = Heap::new();
        let n = row.len();
        let mut count = 0;
        for i in (0..n).into_iter().step_by(2) {
            if !Self::is_couple(row[i], row[i + 1]) {
                count += 2;
            }
        }
        heap.push((count, 0, row));
        while let Some((expect, step, row)) = heap.pop() {
            if expect == step {
                return step;
            }
            let mut idx = vec![0; n];
            for (i, p) in row.iter().enumerate() {
                idx[*p as usize] = i;
            }
            for count in (0..n).into_iter().step_by(2) {
                let x = row[count];
                let y = row[count + 1];
                if !Self::is_couple(x, y) {
                    let p2 = idx[Self::find_half(x) as usize];
                    if p2 > count {
                        let mut row2 = row.clone();
                        let to_swap = Self::find_half(p2 as i32) as usize;
                        if Self::is_couple(y, row[to_swap]) {
                            row2.swap(count, to_swap);
                            heap.push((expect - 3, step + 1, row2));
                            continue;
                        }
                    }
                    let p2 = idx[Self::find_half(y) as usize];
                    if p2 > count {
                        let mut row2 = row.clone();
                        let to_swap = Self::find_half(p2 as i32) as usize;
                        row2.swap(count + 1, to_swap);
                        heap.push((expect - 1, step + 1, row2));
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn test_765() {
    assert_eq!(1, Solution::min_swaps_couples(vec![0, 2, 1, 3]));
    assert_eq!(0, Solution::min_swaps_couples(vec![3, 2, 0, 1]));
}
