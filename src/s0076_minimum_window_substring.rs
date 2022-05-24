struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut count = [0; 256];
        for &c in t.as_bytes() {
            count[c as usize] += 1;
        }

        let n = s.len();
        let m = t.len();
        let mut current = [0; 256];

        if n < m {
            return "".to_string();
        }

        let mut meets = 0;
        let mut lo = 0; // lowest start index
        let mut res = [0, n + 1];

        let bytes = s.as_bytes();

        while lo < n && count[bytes[lo] as usize] == 0 {
            lo += 1;
        }

        let mut j = lo;
        while j < n {
            let bj = bytes[j] as usize;
            if count[bj] > 0 {
                current[bj] += 1;

                if current[bj] > count[bj] {
                    let mut k = lo;
                    loop {
                        let bk = bytes[k] as usize;
                        if current[bk] > count[bk] {
                            current[bk] -= 1;
                            k += 1;
                        } else if count[bk] == 0 {
                            k += 1;
                        } else {
                            lo = k;
                            break;
                        }
                    }
                } else {
                    meets += 1;
                }

                if meets == m && j - lo < res[1] - res[0] {
                    res = [lo, j];
                }
            }
            j += 1;
        }
        if res[1] < n {
            s[res[0]..res[1] + 1].to_string()
        } else {
            "".to_string()
        }
    }
}

#[test]
fn test_0076() {
    let f = |s: &str, t: &str| Solution::min_window(s.to_string(), t.to_string());

    assert_eq!("BANC".to_string(), f("ADOBECODEBANC", "ABC"));
    assert_eq!("a".to_string(), f("a", "a"));
    assert_eq!("".to_string(), f("a", "aa"));
    assert_eq!("b".to_string(), f("ab", "b")); // skip head
    assert_eq!("".to_string(), f("a", "b")); // skip head
}
