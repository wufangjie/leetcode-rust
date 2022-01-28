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
        let mut lst: Vec<(char, i32)> = count.into_iter().collect();
        lst.sort_by(|a, b| b.1.cmp(&a.1));
        if n == 0 || (n + 1 >> 1) < lst[0].1 as usize {
            return String::from("");
        }
        let mut res = String::new();
        let part_n = lst[0].1;
        for part_i in 0..part_n {
            res.push(lst[0].0);
            let mut left = -1;
            for i in 1..lst.len() {
                left += lst[i].1;
                if left >= part_i {
                    res.push(lst[i].0);
                    left -= part_n;
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
