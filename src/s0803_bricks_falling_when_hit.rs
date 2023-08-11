use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Unknown,
    Stable,
    Unstable,
}

type ConnectedPart = (State, VecDeque<(usize, usize)>, HashSet<(usize, usize)>);

struct Solution;

struct Agent<'a> {
    seq: Vec<ConnectedPart>,
    cur: usize,
    valid_idx: Vec<usize>,
    at_the_top: bool,
    count: [usize; 3], // the numbers of unknown/stable/unstable seq
    grid: &'a Vec<Vec<i32>>,
}

impl<'a> Agent<'a> {
    fn new(grid: &'a Vec<Vec<i32>>, i: usize, j: usize) -> Self {
        let seq: Vec<ConnectedPart> = Self::get_next_pos(grid, (i, j), 2)
            .into_iter()
            .map(|ij| {
                let mut visited = HashSet::new();
                visited.insert(ij);
                (State::Unknown, VecDeque::from(vec![ij]), visited)
            })
            .collect();
        let n = seq.len();
        Self {
            seq,
            cur: 0,
            valid_idx: (0..n).collect(),
            at_the_top: i == 0,
            count: [n, 0, 0],
            grid,
        }
    }

    fn get_next_idx(&mut self) -> usize {
        self.cur += 1;
        self.valid_idx[self.cur % self.valid_idx.len()]
    }

    /// part i and j are connected, so we can just merge them
    /// seq[i].0 may be unknown/stable, seq[j].0 always should be unknown
    /// NOTE: return new index
    fn merge(&mut self, i: usize, j: usize) -> usize {
        if self.seq[i].0 == State::Stable {
            self.update_state(j, State::Stable);
            self.count[1] -= 1;
        } else {
            self.count[0] -= 1;
        }
        let hi = i.max(j);
        let lo = i.min(j);
        let item = self.seq.swap_remove(hi);
        self.seq[lo].1.extend(item.1);
        self.seq[lo].2.extend(item.2);
        self.update_valid_index();
        lo
    }

    /// NOTE: state should be Stable or Unstable, self.seq[i].0 should be Unknown
    fn update_state(&mut self, i: usize, state: State) {
        self.seq[i].0 = state;
        self.count[0] -= 1;
        match state {
            State::Stable => self.count[1] += 1,
            State::Unstable => self.count[2] += 1,
            _ => unreachable!(),
        }
    }

    fn update_valid_index(&mut self) {
        self.valid_idx = self
            .seq
            .iter()
            .enumerate()
            .filter(|(_, seq)| seq.0 == State::Unknown)
            .map(|(i, _)| i)
            .collect();
    }

    /// can we totally know all the unstable bricks
    /// the early stopping strategy is the core of this program
    fn is_finished(&self) -> bool {
        if self.at_the_top {
            self.valid_idx.is_empty() // the removed one at row 0
        } else if self.count[0] == 1 {
            self.count[1] == 0 // all of them are unstable except one
        } else {
            self.count[0] == 0
        }
    }

    /// is the position cached, then return the index or None
    fn get_cached_idx(&self, ij: (usize, usize)) -> Option<usize> {
        for (cur, item) in self.seq.iter().enumerate() {
            if item.0 != State::Unstable && item.2.get(&ij).is_some() {
                return Some(cur);
            }
        }
        None
    }

    fn get_next_pos(
        grid: &Vec<Vec<i32>>,
        (i, j): (usize, usize),
        target: i32,
    ) -> Vec<(usize, usize)> {
        let mut poss = Vec::with_capacity(4);
        if i > 0 && grid[i - 1][j] == target {
            poss.push((i - 1, j))
        }
        if j > 0 && grid[i][j - 1] == target {
            poss.push((i, j - 1));
        }
        if i < grid.len() - 1 && grid[i + 1][j] == target {
            poss.push((i + 1, j));
        }
        if j < grid[0].len() - 1 && grid[i][j + 1] == target {
            poss.push((i, j + 1));
        }
        poss
    }

    /// process a position and its neighbors
    fn step(&mut self) {
        let mut cur = self.get_next_idx();
        match self.seq[cur].1.pop_front() {
            Some(ij) if ij.0 == 0 => {
                self.update_state(cur, State::Stable);
                self.update_valid_index();
            }
            Some(ij) => {
                let next_pos = Self::get_next_pos(self.grid, ij, 2);
                for ij in next_pos {
                    match self.get_cached_idx(ij) {
                        Some(x) if x == cur => {}
                        Some(x) => {
                            cur = self.merge(x, cur);
                        }
                        None => {
                            self.seq[cur].1.push_back(ij);
                            self.seq[cur].2.insert(ij);
                        }
                    }
                }
            }
            None => {
                self.update_state(cur, State::Unstable);
                self.update_valid_index();
            }
        }
    }
}

/// there are init unstable bricks
fn init_drop(grid: &mut Vec<Vec<i32>>) {
    let mut stack: Vec<(usize, usize)> = grid[0]
        .iter_mut()
        .enumerate()
        .filter(|(_, &mut v)| v == 1)
        .map(|(i, v)| {
            *v = 2;
            (0, i)
        })
        .collect();

    while let Some((i, j)) = stack.pop() {
        for (ii, jj) in Agent::get_next_pos(grid, (i, j), 1) {
            if grid[ii][jj] == 1 {
                grid[ii][jj] = 2;
                stack.push((ii, jj));
            }
        }
    }
}

impl Solution {
    pub fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        init_drop(&mut grid);
        let mut count = vec![0; hits.len()];

        for (ith, hit) in hits.into_iter().enumerate() {
            let i = hit[0] as usize;
            let j = hit[1] as usize;
            if grid[i][j] < 2 {
                continue;
            }
            grid[i][j] = 0;

            let mut agent = Agent::new(&grid, i, j);
            while !agent.is_finished() {
                agent.step();
            }

            for item in agent.seq {
                if item.0 == State::Unstable {
                    for (i, j) in item.2 {
                        grid[i][j] = 0;
                        count[ith] += 1;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_derive::{Deserialize, Serialize}; //use serde::Deserialize;
                                                // use serde_json;
    use std::fs::File;
    use std::io::Read;

    #[derive(Serialize, Deserialize)]
    struct Data {
        grid: Vec<Vec<i32>>,
        hits: Vec<Vec<i32>>,
    }

    #[test]
    fn test_0803_1() {
        assert_eq!(
            vec![2],
            Solution::hit_bricks(vec![vec![1, 0, 0, 0], vec![1, 1, 1, 0]], vec![vec![1, 0]])
        );

        assert_eq!(
            vec![0, 0, 1, 0],
            Solution::hit_bricks(
                vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]],
                vec![vec![0, 2], vec![2, 0], vec![0, 1], vec![1, 2]]
            )
        );

        assert_eq!(
            vec![0, 0],
            Solution::hit_bricks(
                vec![vec![1, 0, 0, 0], vec![1, 1, 0, 0]],
                vec![vec![1, 1], vec![1, 0]]
            )
        );

        assert_eq!(
            vec![0, 3, 0],
            Solution::hit_bricks(
                vec![vec![1, 0, 1], vec![1, 1, 1]],
                vec![vec![0, 0], vec![0, 2], vec![1, 1]]
            )
        );
    }

    #[test]
    fn test_0803_2() {
        let mut file = File::open("./src/0803.json").expect("Fail to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Fail to read file");
        let data: Data = serde_json::from_str(&contents).expect("Fail to parse json");
        dbg!(data.grid.len());
        dbg!(data.grid[0].len());
        dbg!(data.hits[0].len());

        Solution::hit_bricks(data.grid, data.hits);
    }
}
