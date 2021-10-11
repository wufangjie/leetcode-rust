use leetcode_rust::utils::Stack;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contain_virus(mut is_infected: Vec<Vec<i32>>) -> i32 {
        let height = is_infected.len();
        if height == 0 {
            return 0;
        }
        let width = is_infected[0].len();
        //let mut is_infected_mut = is_infected.clone();

        let mut row_wall = HashSet::new();
        let mut col_wall = HashSet::new();

        let mut count_wall = 0;
        let mut part = HashSet::new();

        loop {
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            let mut threats: Vec<HashSet<(usize, usize)>> = Vec::new();
            let mut best_v = 0;
            let mut best_i = 0;
            let mut best_p = (0, 0);
            let mut part_i = 0;

            let mut best_row_wall_add = HashSet::new();
            let mut best_col_wall_add = HashSet::new();

            let mut to_remove = Vec::new();

            if part.len() == 0 {
                for i in 0..height {
                    for j in 0..width {
                        if is_infected[i][j] == 1 && !visited.contains(&(i, j)) {
                            let mut threat = HashSet::new();
                            let mut row_wall_add = HashSet::new();
                            let mut col_wall_add = HashSet::new();
                            part.insert((i, j));

                            dfs(
                                &is_infected,
                                &mut visited,
                                &row_wall,
                                &col_wall,
                                &mut threat,
                                &mut row_wall_add,
                                &mut col_wall_add,
                                i,
                                j,
                            );
                            if threat.len() > best_v {
                                best_v = threat.len();
                                best_i = part_i;
                                best_p = (i, j);
                                best_row_wall_add = row_wall_add;
                                best_col_wall_add = col_wall_add;
                            }
                            part_i += 1;
                            threats.push(threat);
                        }
                    }
                }
                if part_i > 0 {
                    part.remove(&best_p);
                }
            } else {
                // redundance
                for temp in part.iter() {
                    let i = temp.0;
                    let j = temp.1;
                    let mut threat = HashSet::new();
                    let mut row_wall_add = HashSet::new();
                    let mut col_wall_add = HashSet::new();

                    dfs(
                        &is_infected,
                        &mut visited,
                        &row_wall,
                        &col_wall,
                        &mut threat,
                        &mut row_wall_add,
                        &mut col_wall_add,
                        i,
                        j,
                    );
                    if threat.len() > best_v {
                        best_v = threat.len();
                        best_i = part_i;
                        best_p = (i, j);
                        best_row_wall_add = row_wall_add;
                        best_col_wall_add = col_wall_add;
                    } else if threat.len() == 0 {
                        to_remove.push((i, j));
                    }
                    part_i += 1;
                    threats.push(threat);
                }
                if part_i > 0 {
                    part.remove(&best_p);
                }
                for p in to_remove {
                    part.remove(&p);
                }
            }

            count_wall += best_row_wall_add.len() + best_col_wall_add.len();

            if part.len() == 0 {
                break;
            }
            for (i, j) in best_row_wall_add {
                row_wall.insert((i, j));
            }
            for (i, j) in best_col_wall_add {
                col_wall.insert((i, j));
            }
            for k in 0..threats.len() {
                if k != best_i {
                    for pairs in threats[k].iter() {
                        is_infected[pairs.0][pairs.1] = 1;
                    }
                }
            }
        }
        count_wall as i32
    }
}

fn dfs(
    is_infected: &Vec<Vec<i32>>,
    visited: &mut HashSet<(usize, usize)>,
    row_wall: &HashSet<(usize, usize)>,
    col_wall: &HashSet<(usize, usize)>,
    threat: &mut HashSet<(usize, usize)>,
    row_wall_add: &mut HashSet<(usize, usize)>,
    col_wall_add: &mut HashSet<(usize, usize)>,
    i0: usize,
    j0: usize,
) {
    let height = is_infected.len();
    let width = is_infected[0].len();

    let mut stack = Stack::new();
    stack.push((i0, j0));
    while !stack.is_empty() {
        let temp = stack.pop().unwrap();
        let i = temp.0;
        let j = temp.1;
        if is_infected[i][j] == 0 {
            threat.insert((i, j));
        } else if !visited.contains(&(i, j)) {
            visited.insert((i, j));

            if i > 0 && !row_wall.contains(&(i, j)) {
                if is_infected[i - 1][j] == 0 {
                    row_wall_add.insert((i, j));
                }
                stack.push((i - 1, j));
            }
            if i < height - 1 && !row_wall.contains(&(i + 1, j)) {
                if is_infected[i + 1][j] == 0 {
                    row_wall_add.insert((i + 1, j));
                }
                stack.push((i + 1, j));
            }
            if j > 0 && !col_wall.contains(&(i, j)) {
                if is_infected[i][j - 1] == 0 {
                    col_wall_add.insert((i, j));
                }
                stack.push((i, j - 1));
            }
            if j < width - 1 && !col_wall.contains(&(i, j + 1)) {
                if is_infected[i][j + 1] == 0 {
                    col_wall_add.insert((i, j + 1));
                }
                stack.push((i, j + 1));
            }
        }
    }
}

#[test]
fn test() {
    let is_infected: Vec<Vec<i32>> = vec![
        vec![0, 1, 0, 0, 0, 0, 0, 1],
        vec![0, 1, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::contain_virus(is_infected), 10);

    let is_infected: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    assert_eq!(Solution::contain_virus(is_infected), 4);

    let is_infected: Vec<Vec<i32>> = vec![
        vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 1, 0, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::contain_virus(is_infected), 13);

    let is_infected: Vec<Vec<i32>> = vec![
        vec![0, 1, 0, 0, 0, 0, 0, 1],
        vec![0, 1, 0, 1, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 1],
    ];
    assert_eq!(Solution::contain_virus(is_infected), 16);

    let is_infected: Vec<Vec<i32>> = vec![vec![0]];
    assert_eq!(Solution::contain_virus(is_infected), 0);

    let is_infected: Vec<Vec<i32>> = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 1, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::contain_virus(is_infected), 56);

    let is_infected: Vec<Vec<i32>> = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::contain_virus(is_infected), 205);
}
