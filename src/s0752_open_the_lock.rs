use leetcode_rust::utils::Heap;
use std::collections::HashSet;

struct Solution;

pub fn num4_to_nums(num4: [i8; 4]) -> u16 {
    1000 * (num4[0] as u16) + 100 * (num4[1] as u16) + 10 * (num4[2] as u16) + (num4[3] as u16)
}

pub fn nums_to_num4(nums: u16) -> [i8; 4] {
    [
        (nums / 1000) as i8,
        (nums / 100 % 10) as i8,
        (nums / 10 % 10) as i8,
        (nums % 10) as i8,
    ]
}

pub fn min<T: PartialOrd + Clone>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

impl Solution {
    pub fn open_lock(deadends: Vec<&str>, target: &str) -> i32 {
        // for local test
        //pub fn open_lock(deadends: Vec<String>, target: String) -> i32 { // for web
        let mut deadsets: HashSet<u16> = HashSet::new();
        for s in deadends {
            deadsets.insert(s.parse().unwrap());
        }
        if deadsets.contains(&0) {
            return -1;
        }

        let target4 = nums_to_num4(target.parse().unwrap());

        let expect = |step: usize, num4: [i8; 4]| -> (usize, usize, [i8; 4]) {
            let mut expt = step;
            for i in 0..4 {
                let di: i8 = (num4[i] - target4[i]).abs();
                expt += min(di, 10 - di) as usize;
            }
            (expt, step, num4)
        };

        let mut heap = Heap::new();
        heap.push(expect(0, [0; 4]));
        deadsets.insert(0);
        while !heap.is_empty() {
            let (_expt, step, num4) = heap.pop().unwrap();
            if num4 == target4 {
                return step as i32;
            }

            for i in 0..4 {
                let mut near = num4;
                let mut j = 1;
                while j < 10 {
                    near[i] = (num4[i] + j) % 10;
                    j += 8;
                    let key = num4_to_nums(near);
                    if !deadsets.contains(&key) {
                        deadsets.insert(key);
                        heap.push(expect(step + 1, near));
                    }
                }
            }
        }
        return -1;
    }
}

#[test]
fn test() {
    // println!("{}, {}", -1 % 10, 12 % 10);
    assert_eq!(
        Solution::open_lock(vec!["0201", "0101", "0102", "1212", "2002"], "0202"),
        6
    );
    assert_eq!(Solution::open_lock(vec!["8888"], "0009"), 1);
    assert_eq!(
        Solution::open_lock(
            vec!["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"],
            "8888"
        ),
        -1
    );
}
