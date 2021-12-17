use utils::Bisect;

struct Solution;

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let n = positions.len();
        let mut parts = Vec::with_capacity(n);
        parts.push((i32::MIN, 0));
        parts.push((i32::MAX, 0));

        let mut highest = 0;
        let mut res = vec![];
        for square in positions {
            let left = square[0];
            let length = square[1];
            let i = parts.bisect_right((left, i32::MAX));
            let j = parts.bisect_left((left + length, i32::MIN));

            let h = length + parts[i - 1..j].iter().map(|(_, h)| *h).max().unwrap();
            if h > highest {
                highest = h;
            }
            res.push(highest);
            if i == j {
                parts.insert(j, (left + length, parts[j - 1].1));
                parts.insert(i, (left, h));
            } else {
                parts[j - 1].0 = left + length;
                if j == i + 1 {
                    parts.insert(i, (left, h));
                } else {
                    parts[i] = (left, h);
                    drop(parts.drain(i + 1..j - 1));
                }
            }
        }
        res
    }
}

#[test]
fn test_699() {
    assert_eq!(
        vec![2, 5, 5],
        Solution::falling_squares(vec![vec![1, 2], vec![2, 3], vec![6, 1]])
    );
}
