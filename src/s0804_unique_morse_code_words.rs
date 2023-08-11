use std::collections::HashSet;
use std::iter::FromIterator; // prelude since rust_2021

struct Solution;

const TABLE: [&str; 26] = [
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
    "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
];

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        HashSet::<String>::from_iter(words.into_iter().map(|word| {
            word.bytes()
                .map(|c| TABLE[(c - b'a') as usize])
                .fold(String::new(), |mut acc, x| {
                    acc.push_str(x);
                    acc
                })
        }))
        .len() as i32
    }
}

// impl Solution {
//     pub fn unique_morse_representations(words: Vec<String>) -> i32 {
//         let mut set = HashSet::new();
//         words.into_iter().for_each(|word| {
//             set.insert(word.bytes().map(|c| TABLE[(c - b'a') as usize]).fold(
//                 String::new(),
//                 |mut acc, x| {
//                     acc.push_str(x);
//                     acc
//                 },
//             ));
//         });
//         set.len() as i32
//     }
// }

#[test]
fn test_0804() {
    assert_eq!(
        2,
        Solution::unique_morse_representations(
            vec!["gin", "zen", "gig", "msg"]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        )
    );
}
