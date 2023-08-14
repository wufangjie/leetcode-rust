use std::cmp::Ordering;
use std::str::Bytes;

struct Solution;

impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let scc: Vec<CharCount> = CharCountIter::new(&s).collect();
        words
            .into_iter()
            .map(|word| scc.iter().copied().eq(CharCountIter::new(&word)) as i32)
            .sum()
    }
}

#[derive(Clone, Copy)]
struct CharCount {
    ch: u8,
    cnt: usize,
}

impl CharCount {
    fn new(ch: u8, cnt: usize) -> Self {
        Self { ch, cnt }
    }
}

impl PartialEq for CharCount {
    fn eq(&self, other: &Self) -> bool {
        if self.ch == other.ch {
            match self.cnt.cmp(&other.cnt) {
                Ordering::Equal => true,
                Ordering::Greater => self.cnt >= 3,
                Ordering::Less => false,
            }
        } else {
            false
        }
    }
}

struct CharCountIter<'a> {
    s: Bytes<'a>,
    c: Option<u8>,
}

impl<'a> CharCountIter<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            s: s.bytes(),
            c: None,
        }
    }
}

impl Iterator for CharCountIter<'_> {
    type Item = CharCount;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c1) = self.c.take() {
            let mut count = 1;
            while let Some(c2) = self.s.next() {
                if c2 == c1 {
                    count += 1;
                } else {
                    self.c = Some(c2);
                    break;
                }
            }
            Some(CharCount::new(c1, count))
        } else if let Some(c2) = self.s.next() {
            self.c = Some(c2);
            self.next()
        } else {
            None
        }
    }
}

#[test]
fn test_0808() {
    assert_eq!(
        1,
        Solution::expressive_words(
            "heeellooo".to_string(),
            vec!["hello", "hi", "helo"]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        )
    );
    assert_eq!(
        3,
        Solution::expressive_words(
            "zzzzzyyyyy".to_string(),
            vec!["zzyy", "zy", "zyy"]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        )
    );
}
