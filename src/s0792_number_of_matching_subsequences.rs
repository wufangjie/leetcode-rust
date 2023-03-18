struct Solution;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut index = vec![vec![]; 26];
        for (i, c) in s.bytes().enumerate() {
            index[(c - b'a') as usize].push(i as i32);
        }
        let mut count = 0;
        'outer: for word in words {
            let mut idx_gt = -1;
            for c in word.as_bytes() {
                let to_search = &index[(c - b'a') as usize];
                match to_search.binary_search(&idx_gt) {
                    Err(i) => {
                        if i < to_search.len() {
                            idx_gt = to_search[i];
                        } else {
                            continue 'outer
                        }
                    },
                    Ok(i) => {
                        if i + 1 < to_search.len() {
                            idx_gt = to_search[i + 1];
                        } else {
                            continue 'outer
                        }
                    },
                }
            }
            count += 1;
        }
        count
    }
}
