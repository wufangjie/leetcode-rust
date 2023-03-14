use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Solution;

#[derive(PartialEq, Eq)]
struct Pair<'a> {
    arr: &'a [i32],
    i: usize,
    j: usize,
}

impl PartialOrd for Pair<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.arr[self.j] * self.arr[other.i]).cmp(&(self.arr[self.i] * self.arr[other.j])))
    }
}

impl Ord for Pair<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        let n = arr.len() - 1;
        heap.push(Pair { arr: &arr, i: 0, j: n });
        for _ in 0..k - 1 {
            let Pair { i, j, .. } = heap.pop().unwrap();
            if j - i > 1 {
                if i == 0 || arr[j - 1] * arr[i] < arr[i - 1] * arr[j] {
                    heap.push(Pair { arr: &arr, i, j: j - 1 });
                }
                if j == n || arr[j + 1] * arr[i] < arr[i + 1] * arr[j] {
                    heap.push(Pair { arr: &arr, i: i + 1, j });
                }
            }
        }
        let Pair { i, j, .. } = heap.pop().unwrap();
        return vec![arr[i] as i32, arr[j] as i32];
    }
}

#[test]
fn test_0786() {
    dbg!(Solution::kth_smallest_prime_fraction(
        vec![1, 7, 23, 29, 47],
        8
    ));
}
