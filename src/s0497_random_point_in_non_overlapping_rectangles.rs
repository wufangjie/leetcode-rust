use rand::Rng;
use utils::Bisect;

#[derive(Debug)]
struct Solution {
    area_lst: Vec<u64>,
    help_lst: Vec<(i32, i32, u64)>,
    max: u64,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let n = rects.len();
        let mut area_lst = Vec::with_capacity(n);
        let mut pre = 0;
        let mut help_lst = Vec::with_capacity(n); // [x0, y0, width]
        for r in rects.into_iter() {
            let width = (r[2] - r[0] + 1) as u64;
            pre += ((r[3] - r[1] + 1) as u64) * width;
            area_lst.push(pre);
            help_lst.push((r[0], r[1], width));
        }

        Self {
            area_lst,
            help_lst,
            max: pre,
        }
    }

    fn pick(&self) -> Vec<i32> {
        let mut i = rand::thread_rng().gen_range(0..self.max);
        let j = self.area_lst.bisect_right(i);
        let (x0, y0, ww) = self.help_lst[j];
        if j > 0 {
            i -= self.area_lst[j - 1];
        }
        vec![x0 + (i % ww) as i32, y0 + (i / ww) as i32]
    }
}

#[test]
fn test() {
    let s = Solution::new(vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]]);
    for _ in 0..10 {
        dbg!(s.pick());
    }
}
