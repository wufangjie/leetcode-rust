struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut total = 1;
        for c in s.bytes() {
            if c >= b'A' {
                total <<= 1;
            }
        }
        let s2 = s.clone(); // it's important, out of vec![]
        let mut res = vec![s2; total];
        let mut n = total;
        for (pos, c) in s.bytes().enumerate() {
            match c {
                b'A'..=b'Z' => {
                    n >>= 1;
                    let lower = c.to_ascii_lowercase();
                    for (i, si) in res.iter_mut().enumerate() {
                        if (i / n) & 1 == 0 {
                            unsafe { si.as_mut_vec()[pos] = lower };
                        }
                    }
                },
                b'a'..=b'z' => {
                    n >>= 1;
                    let upper = c.to_ascii_uppercase();
                    for (i, si) in res.iter_mut().enumerate() {
                        if (i / n) & 1 == 1 {
                            unsafe { si.as_mut_vec()[pos] = upper };
                        }
                    }
                }
                _ => (),
            }
        }
        res
    }
}
