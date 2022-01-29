use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let n = s.len();
        let mut count: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            let p = count.entry(c).or_insert(0);
            *p += 1;
        }

        let mut max_k = ' ';
        let mut max_c = 0;
        for (&k, &c) in count.iter() {
            if c > max_c {
                max_c = c;
                max_k = k;
            }
        }
        if n == 0 || (n + 1 >> 1) < max_c as usize {
            return String::from("");
        }

        count.remove(&max_k);
        let mut res = String::new();
        for part_i in 0..max_c {
            res.push(max_k);
            let mut left = -1;
            for (&k, &c) in count.iter() {
                left += c;
                if left >= part_i {
                    res.push(k);
                    left -= max_c;
                }
            }
        }

        res
    }
}

#[test]
fn test_767() {
    assert_eq!(
        String::from("aba"),
        Solution::reorganize_string(String::from("aab"))
    );
    assert_eq!(
        String::from("vlvov"),
        Solution::reorganize_string(String::from("vvvlo"))
    );
}
