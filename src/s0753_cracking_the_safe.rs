use leetcode_rust::utils::Stack;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        let mut s = String::from("");
        let k = k as u8;
        if n == 1 {
            for i in 48u8..(48u8 + k) {
                s.push(i as char);
            }
            return s;
        }
        let mut visited = HashSet::new();
        visited.insert(0u16);

        let total = (k as usize).pow(n as u32);
        let base = 10u16.pow(n as u32 - 1);
        let mut stack: Stack<(u16, HashSet<u16>, String)> = Stack::new();
        stack.push((0u16, visited, format!("{:>0width$}", 0, width = n as usize)));

        loop {
            let (pre, visited, ret) = stack.pop().unwrap();
            if visited.len() == total {
                return ret;
            }
            let pre10 = pre * 10;

            for i in 0..k {
                let key = pre10 + i as u16;
                if !visited.contains(&key) {
                    let mut v2 = visited.clone();
                    v2.insert(key);
                    let mut r2 = ret.clone();
                    r2.push((i + 48) as char);
                    stack.push((key % base, v2, r2));
                }
            }
        }
    }
}

#[test]
fn test() {
    println!("{:?}", Solution::crack_safe(1, 5));
    println!("{:#?}", Solution::crack_safe(2, 2));
    println!("{:#?}", Solution::crack_safe(3, 4));
}
