use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn get_skyline(mut buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        buildings.sort_unstable_by(|xyh1, xyh2| (xyh1[0], -xyh1[2]).cmp(&(xyh2[0], -xyh2[2])));
        let mut heap = BinaryHeap::new();
        heap.push((0, i32::MAX));
        let mut res = vec![];
        let mut have_i32_max = false;
        for xyh in buildings {
            have_i32_max |= xyh[1] == i32::MAX;
            Self::pop_until(&mut heap, &mut res, xyh[0]);
            let &(h, r) = heap.peek().unwrap();
            if xyh[2] == h {
                if xyh[1] > r {
                    *heap.peek_mut().unwrap() = (h, xyh[1]);
                }
            } else if xyh[2] > h {
                res.push(vec![xyh[0], xyh[2]]);
                heap.push((xyh[2], xyh[1]));
            } else if xyh[1] > r {
                heap.push((xyh[2], xyh[1]));
            }
        }
        Self::pop_until(&mut heap, &mut res, i32::MAX);
        if have_i32_max {
            res.push(vec![i32::MAX, 0]);
        }
        res
    }

    fn pop_until(heap: &mut BinaryHeap<(i32, i32)>, res: &mut Vec<Vec<i32>>, threshold: i32) {
        loop {
            let &(h, r) = heap.peek().unwrap();
            match res.len() {
                0 => (),
                n => {
                    if r > res[n - 1][0] {
                        if res[n - 1][1] == -1 {
                            res[n - 1][1] = h;
                        }
                        res.push(vec![r, -1]);
                    }
                }
            }
            if r >= threshold {
                res.pop();
                return;
            } else {
                heap.pop();
            }
        }
    }
}

#[test]
fn test_218() {
    assert_eq!(
        Solution::get_skyline(vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8]
        ]),
        vec![
            vec![2, 10],
            vec![3, 15],
            vec![7, 12],
            vec![12, 0],
            vec![15, 10],
            vec![20, 8],
            vec![24, 0]
        ]
    );
    assert_eq!(
        Solution::get_skyline(vec![vec![2, 9, 10], vec![2, 9, 11]]),
        vec![vec![2, 11], vec![9, 0]]
    );
    assert_eq!(
        Solution::get_skyline(vec![vec![2, 9, 10], vec![3, 9, 1]]),
        vec![vec![2, 10], vec![9, 0]]
    );
    assert_eq!(
        Solution::get_skyline(vec![vec![0, 2, 3], vec![2, 5, 3]]),
        vec![vec![0, 3], vec![5, 0]]
    );
    assert_eq!(
        Solution::get_skyline(vec![
            vec![3, 7, 8],
            vec![3, 8, 7],
            vec![3, 9, 6],
            vec![3, 10, 5],
            vec![3, 11, 4],
            vec![3, 12, 3],
            vec![3, 13, 2],
            vec![3, 14, 1]
        ]),
        vec![
            vec![3, 8],
            vec![7, 7],
            vec![8, 6],
            vec![9, 5],
            vec![10, 4],
            vec![11, 3],
            vec![12, 2],
            vec![13, 1],
            vec![14, 0]
        ]
    );
    assert_eq!(
        Solution::get_skyline(vec![
            vec![3, 10, 20],
            vec![3, 9, 19],
            vec![3, 8, 18],
            vec![3, 7, 17],
            vec![3, 6, 16],
            vec![3, 5, 15],
            vec![3, 4, 14]
        ]),
        vec![vec![3, 20], vec![10, 0]]
    );

    assert_eq!(
        Solution::get_skyline(vec![vec![0, 2147483647, 2147483647]]),
        vec![vec![0, 2147483647], vec![2147483647, 0]]
    );
}
