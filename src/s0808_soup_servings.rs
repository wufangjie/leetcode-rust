struct Solution;

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n >= 5000 {
            return 1.0;
        }
        let mut dp = DP::new(n);
        dp.process_all()
    }
}

struct DP {
    table: Vec<Vec<f64>>,
    reminder: i32,
}

impl DP {
    fn new(n: i32) -> Self {
        let reminder = n % 25;
        let round = n as usize / 25 + 1;
        let table = vec![vec![0.0; round]; round];
        Self { table, reminder }
    }

    fn process_all(&mut self) -> f64 {
        let n = self.table.len() as i32;
        let lo = if self.reminder == 0 {
            self.table[0][0] = 0.5;
            self.table[0].iter_mut().skip(1).for_each(|x| *x = 1.0);
            1
        } else {
            0
        };

        for i in lo..n {
            for j in lo..n {
                self.process_one(i, j);
            }
        }
        let n = n as usize - 1;
        self.table[n][n]
    }

    fn process_one(&mut self, i: i32, j: i32) {
        self.table[i as usize][j as usize] =
            (0..4).map(|x| self.get_prob(i, j, x)).sum::<f64>() / 4.0;
    }

    fn get_prob(&self, i: i32, j: i32, op: i32) -> f64 {
        let i2 = i - 4 + op;
        let j2 = j - op;
        match i2 {
            0 => {
                if self.reminder > 0 {
                    if j2 < 0 {
                        0.0
                    } else {
                        self.table[i2 as usize][j2 as usize]
                    }
                } else if j2 <= 0 {
                    0.5
                } else {
                    self.table[i2 as usize][j2 as usize]
                }
            }
            x if x < 0 => {
                if j2 * 25 + self.reminder <= 0 {
                    0.5
                } else {
                    1.0
                }
            }
            _ => {
                if j2 * 25 + self.reminder <= 0 {
                    0.0
                } else {
                    self.table[i2 as usize][j2 as usize]
                }
            }
        }
    }
}

#[test]
fn test_0808() {
    dbg!(Solution::soup_servings(50));
    dbg!(Solution::soup_servings(100));
    dbg!(Solution::soup_servings(75));
    for i in 5000..5025 {
        dbg!(Solution::soup_servings(i));
    }
}
