struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last = [0; 256];
        let mut curr = 0;
        let mut longest = 0;
        for (mut i, &c) in s.as_bytes().iter().enumerate() {
            let c = c as usize;
            i += 1;
            if last[c] > curr {
                curr = last[c];
            }
            if i - curr > longest {
                longest = i - curr;
            }
            last[c] = i;
        }
        longest as i32
    }
}

#[test]
fn test_0003() {
    assert_eq!(
        3,
        Solution::length_of_longest_substring("abcabcbb".to_string())
    );
    assert_eq!(
        1,
        Solution::length_of_longest_substring("bbbbb".to_string())
    );
    assert_eq!(
        3,
        Solution::length_of_longest_substring("pwwkew".to_string())
    );
    assert_eq!(1, Solution::length_of_longest_substring(" ".to_string()));
    assert_eq!(0, Solution::length_of_longest_substring("".to_string()));
}
