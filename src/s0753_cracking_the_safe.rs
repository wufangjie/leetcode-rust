struct Solution;

impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        let mut s = format!("{:>0width$}", 0, width = n as usize - 1);
        if n == 1 {
            for i in 49u8..(48u8 + k as u8) {
                s.push(i as char);
            }
            return s;
        }
        let k = k as usize;
        let total = k.pow(n as u32) as u16;
        let mut visited = vec![0u16; total as usize];
        visited[0] = 1;
        let base = k.pow(n as u32 - 1) as u16;
        let mut stack = vec![(0u16, 1u16, visited)];

        loop {
            let (pre, count, visited) = stack.pop().unwrap();
            if count == total {
                let mut temp: Vec<(usize, u16)> = visited.into_iter().enumerate().collect();
                temp.sort_by(|a, b| a.1.cmp(&b.1));
                for (i, _) in temp {
                    s.push(((i % k) as u8 + 48) as char);
                }
                return s;
            }
            let pre_k = pre as usize * k;
            for i in pre_k..pre_k + k {
                if visited[i] == 0 {
                    // Euler's path exist!
                    let mut v2 = visited.clone();
                    v2[i] = count + 1;
                    stack.push(((i as u16) % base, count + 1, v2));
                }
            }
        }
    }
}

#[test]
fn test_753() {
    println!("{:?}", Solution::crack_safe(1, 5));
    println!("{:#?}", Solution::crack_safe(2, 2));
    println!("{:#?}", Solution::crack_safe(3, 4));
}
