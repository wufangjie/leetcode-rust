use leetcode_rust::utils::Heap;

struct Solution;

pub fn num4_to_nums(num4: [i8; 4]) -> usize {
    1000 * (num4[0] as usize)
        + 100 * (num4[1] as usize)
        + 10 * (num4[2] as usize)
        + (num4[3] as usize)
}

pub fn nums_to_num4(nums: usize) -> [i8; 4] {
    [
        (nums / 1000) as i8,
        (nums / 100 % 10) as i8,
        (nums / 10 % 10) as i8,
        (nums % 10) as i8,
    ]
}

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut visited = vec![false; 10000];

        for s in deadends {
            if s == "0000" {
                return -1;
            }
            visited[s.parse::<usize>().unwrap()] = true;
        }

        let target4 = nums_to_num4(target.parse().unwrap());
        let expect = |step: i32, num4: [i8; 4]| -> (i32, i32, [i8; 4]) {
            let mut expt = step;
            for i in 0..4 {
                expt += match num4[i] - target4[i] {
                    i if i > 5 => 10 - i,
                    i if i > 0 => i,
                    i if i > -5 => -i,
                    i => 10 + i,
                } as i32;
            }
            (expt, step, num4)
        };

        let mut heap = Heap::new();
        heap.push(expect(0, [0; 4]));
        visited[0] = true;
        while let Some((expt, step, num4)) = heap.pop() {
            if expt == step {
                return step;
            }
            for i in 0..4 {
                let mut near = num4;
                let mut j = 1;
                while j < 10 {
                    near[i] = (num4[i] + j) % 10;
                    j += 8; // -1
                    let key = num4_to_nums(near);
                    if !visited[key] {
                        visited[key] = true;
                        heap.push(expect(step + 1, near));
                    }
                }
            }
        }
        return -1;
    }
}

#[test]
fn test_752() {
    assert_eq!(
        Solution::open_lock(
            vec!["0201", "0101", "0102", "1212", "2002"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
            "0202".to_owned()
        ),
        6
    );
    assert_eq!(
        Solution::open_lock(
            vec!["8888"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
            "0009".to_owned()
        ),
        1
    );
    assert_eq!(
        Solution::open_lock(
            vec!["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
            "8888".to_owned()
        ),
        -1
    );
}
