use rand::{thread_rng, Rng};
use std::collections::HashMap;

struct Solution {
    flipped: HashMap<i32, i32>,
    left: i32,
    total: usize,
    n: i32,
    rng: rand::rngs::ThreadRng,
}

impl Solution {
    fn new(m: i32, n: i32) -> Self {
        let left = m * n;
        let total = left as usize;
        Self {
            flipped: HashMap::with_capacity(total.min(1024)), // not too big
            left,
            total,
            n,
            rng: thread_rng(),
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        //let x: i32 = self.rng.gen_range(0, self.left);
        let x: i32 = self.rng.gen_range(0..self.left);
        if self.left == 0 {
            return vec![];
        }
        self.left -= 1;
        let ret = match self.flipped.get(&x) {
            Some(&y) => y,
            None => x,
        };
        if x < self.left {
            match self.flipped.get(&self.left) {
                Some(&z) => self.flipped.insert(x, z),
                None => self.flipped.insert(x, self.left),
            };
        }
        vec![ret / self.n, ret % self.n]
    }

    fn reset(&mut self) {
        self.flipped.clear();
        self.left = self.total as i32;
    }
}

#[test]
fn test_0519() {
    let _s = Solution::new(10000, 10000);
    // it's ok locally, but caused Runtime Error on leetcode
    // finally, I found it cost too much memory
    // so I add a 1024 super bound:
    // flipped: HashMap::with_capacity(total.min(1024))
}
