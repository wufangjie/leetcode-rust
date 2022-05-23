use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut count = HashMap::<&str, usize>::with_capacity(words.len());
        for word in words.iter() {
            *count.entry(word).or_insert(0) += 1;
        }

        let n = s.len();
        let m = words.len();
        let w = words[0].len();
        let mut current = HashMap::new();
        let mut res: Vec<i32> = vec![];

        if n < m * w {
            return vec![];
        }

        for i in 0..w {
            let mut j = i;
            let mut meets = 0;
            current.clear();

            while j <= n - w {
                if count.contains_key(&s[j..j + w]) {
                    let p = current.entry(&s[j..j + w]).or_insert_with(VecDeque::new);
                    (*p).push_back(j);
                    meets += 1;
                    if (*p).len() > *count.get(&s[j..j + w]).unwrap() {
                        let mut k = j - (meets - 1) * w;
                        loop {
                            current.get_mut(&s[k..k + w]).unwrap().pop_front();
                            meets -= 1;
                            if s[k..k + w] == s[j..j + w] {
                                break;
                            }
                            k += w;
                        }
                    }
                } else {
                    current.clear();
                    meets = 0;
                }

                if meets == m {
                    res.push((j - (m - 1) * w) as i32);
                }
                j += w;
            }
        }
        res
    }
}

#[test]
fn test_0030() {
    let f = |s: &str, words: Vec<&str>| {
        Solution::find_substring(
            s.to_string(),
            words
                .into_iter()
                .map(|w| w.to_string())
                .collect::<Vec<String>>(),
        )
    };

    let mut res = f("barfoothefoobarman", vec!["foo", "bar"]);
    res.sort_unstable();
    assert_eq!(vec![0, 9], res);

    let res = f(
        "wordgoodgoodgoodbestword",
        vec!["word", "good", "best", "word"],
    );
    assert!(res.is_empty());

    let mut res = f("barfoofoobarthefoobarman", vec!["bar", "foo", "the"]);
    res.sort_unstable();
    assert_eq!(vec![6, 9, 12], res);

    // while j <= n - w {
    let mut res = f(
        "wordgoodgoodgoodbestword",
        vec!["word", "good", "best", "good"],
    );
    res.sort_unstable();
    assert_eq!(vec![8], res);
}
