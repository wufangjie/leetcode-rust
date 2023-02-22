use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet};

struct Solution {
    max: i32,
    map: HashMap<i32, i32>,
    rng: rand::rngs::ThreadRng,
}

impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let m = blacklist.len();
        let mut i = n - m as i32;
        let max = i;
        let blacklist = blacklist.into_iter().collect::<HashSet<i32>>();
        let mut map = HashMap::with_capacity(m.min(max as usize));
        for &k in &blacklist {
            if k < max {
                while blacklist.contains(&i) {
                    i += 1;
                }
                map.insert(k, i);
                i += 1;
            }
        }
        Self {
            max,
            map,
            rng: thread_rng(),
        }
    }

    fn pick(&mut self) -> i32 {
        // let i = self.rng.gen_range(0, self.max); // leetcode used the older version, so the api is different
        let i = self.rng.gen_range(0..self.max);
        match self.map.get(&i) {
            None => i,
            Some(v) => *v,
        }
    }
}

#[test]
fn test_0710() {
    let mut s = Solution::new(7, vec![2, 3, 5]);
    for _ in 0..20 {
        dbg!(s.pick());
    }
}
