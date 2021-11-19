use std::collections::HashSet;

struct Solution;

impl Solution {
    fn find_outline_by_outline(
        is_infected: &Vec<Vec<i32>>,
        row_wall: &HashSet<(u8, u8)>,
        col_wall: &HashSet<(u8, u8)>,
        idx: i32,
        ij: (u8, u8),
        visited: &mut Vec<Vec<bool>>, //HashSet<(u8, u8)>,
    ) -> (HashSet<(u8, u8)>, Vec<(u8, u8)>, Vec<(u8, u8)>) {
        let height = is_infected.len() as u8;
        let width = is_infected[0].len() as u8;
        let mut outline = HashSet::new();
        let mut row_wall_add = vec![];
        let mut col_wall_add = vec![];
        let mut stack = vec![ij];
        while let Some((i, j)) = stack.pop() {
            let ii = i as usize;
            let jj = j as usize;
            if i > 0 && !row_wall.contains(&(i, j)) {
                if is_infected[ii - 1][jj] == 0 || is_infected[ii - 1][jj] == idx {
                    outline.insert((i - 1, j));
                    row_wall_add.push((i, j));
                } else if !visited[ii - 1][jj] {
                    stack.push((i - 1, j));
                }
                visited[ii - 1][jj] = true;
            }
            if i < height - 1 && !row_wall.contains(&(i + 1, j)) {
                if is_infected[ii + 1][jj] == 0 || is_infected[ii + 1][jj] == idx {
                    outline.insert((i + 1, j));
                    row_wall_add.push((i + 1, j));
                } else if !visited[ii + 1][jj] {
                    stack.push((i + 1, j));
                }
                visited[ii + 1][jj] = true;
            }
            if j > 0 && !col_wall.contains(&(i, j)) {
                if is_infected[ii][jj - 1] == 0 || is_infected[ii][jj - 1] == idx {
                    outline.insert((i, j - 1));
                    col_wall_add.push((i, j));
                } else if !visited[ii][jj - 1] {
                    stack.push((i, j - 1));
                }
                visited[ii][jj - 1] = true;
            }
            if j < width - 1 && !col_wall.contains(&(i, j + 1)) {
                if is_infected[ii][jj + 1] == 0 || is_infected[ii][jj + 1] == idx {
                    outline.insert((i, j + 1));
                    col_wall_add.push((i, j + 1));
                } else if !visited[ii][jj + 1] {
                    stack.push((i, j + 1));
                }
                visited[ii][jj + 1] = true;
            }
        }
        (outline, row_wall_add, col_wall_add)
    }

    pub fn contain_virus(mut is_infected: Vec<Vec<i32>>) -> i32 {
        let height = is_infected.len();
        if height == 0 {
            return 0;
        }
        let width = is_infected[0].len();

        let mut row_wall = HashSet::new();
        let mut col_wall = HashSet::new();
        let mut ret = 0;
        let mut idx = 2;

        loop {
            let mut n_max = 0;
            let mut outline_i = HashSet::new();
            let mut row_wall_add_i = vec![];
            let mut col_wall_add_i = vec![];
            let mut visited = vec![vec![false; width]; height];

            for i in 0..height {
                for j in 0..width {
                    let ij = (i as u8, j as u8);
                    if is_infected[i][j] != idx - 1 || visited[i][j] {
                        continue;
                    }
                    visited[i][j] = true;

                    let (outline, row_wall_add, col_wall_add) = Self::find_outline_by_outline(
                        &is_infected,
                        &row_wall,
                        &col_wall,
                        idx,
                        ij,
                        &mut visited,
                    );
                    let n = outline.len();

                    if n > n_max {
                        n_max = n;
                        row_wall_add_i = row_wall_add;
                        col_wall_add_i = col_wall_add;
                        for (x, y) in outline_i {
                            is_infected[x as usize][y as usize] = idx;
                        }
                        outline_i = outline;
                    } else {
                        for (x, y) in outline {
                            is_infected[x as usize][y as usize] = idx;
                        }
                    }
                }
            }

            if n_max == 0 {
                return ret as i32;
            }
            idx += 1;
            ret += row_wall_add_i.len() + col_wall_add_i.len();
            // println!("{}", ret);
            // for row in &is_infected {
            //     println!("{:?}", row);
            // }
            for xy in row_wall_add_i {
                row_wall.insert(xy);
            }
            for xy in col_wall_add_i {
                col_wall.insert(xy);
            }
        }
    }
}

#[test]
fn test_749() {
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

    let is_infected: Vec<Vec<i32>> = vec![
        vec![0, 1, 0, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 0, 0, 1, 0, 1, 1, 0, 1],
        vec![0, 0, 0, 1, 0, 1, 0, 1, 1, 1],
        vec![0, 1, 0, 0, 1, 0, 0, 1, 1, 0],
        vec![0, 1, 0, 1, 0, 0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 1, 0, 0, 1],
        vec![1, 0, 1, 1, 0, 1, 0, 1, 0, 1],
    ];
    assert_eq!(Solution::contain_virus(is_infected), 38);
}
