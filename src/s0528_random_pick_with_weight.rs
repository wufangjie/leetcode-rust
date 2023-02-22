use rand::{Rng, thread_rng};

struct Solution {
    cumsum: Vec<i32>,
    max: i32,
    rng: rand::rngs::ThreadRng,
}

impl Solution {

    fn new(w: Vec<i32>) -> Self {
        let n = w.len();
        let mut cumsum = Vec::with_capacity(n);
        let mut max = 0;
        for i in 0..n{
            max += w[i];
            cumsum.push(max);
        }
        Self {
            cumsum,
            max,
            rng: thread_rng()
        }
    }

    fn pick_index(&mut self) -> i32 {
        let r = self.rng.gen_range(0, self.max);
        (match self.cumsum.binary_search(&r) {
            Ok(i) => i + 1,
            Err(i) => i
        }) as i32
    }
}

/// NOTE: rust's binary_search returns Result<usize, usize> is fast,
/// no need to write following code
// fn bisect(lst: &[i32], r: i32) -> usize {
//     let mut lo = 0;
//     let mut hi = lst.len();
//     while lo < hi {
//         let mid = (lo + hi) >> 1;
//         if lst[mid] == r {
//             return mid + 1;
//         } else if lst[mid] < r {
//             lo = mid + 1;
//         } else {
//             hi = mid;
//         }
//     }
//     lo
// }
