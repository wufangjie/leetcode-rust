use leetcode_rust::utils::Stack;
use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    //pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
    pub fn pyramid_transition(bottom: String, allowed: Vec<&str>) -> bool {
        let mut allowed_map: HashMap<&str, Vec<char>> = HashMap::new();
        for s in allowed.iter() {
            allowed_map
                .entry(&s[..2])
                .or_insert(vec![])
                .push((&s[2..3]).parse().unwrap());
        }

        let mut cache: HashSet<String> = HashSet::new();
        let mut bottom = bottom;
        let mut stack = Stack::new();
        stack.push(bottom);
        'main: while !stack.is_empty() {
            bottom = stack.pop().unwrap();
            if bottom.len() == 1 {
                return true;
            }
            let mut poss: Vec<&Vec<char>> = vec![];
            for i in 0..(bottom.len() - 1) {
                match allowed_map.get(&bottom[i..i + 2]) {
                    Some(p) => poss.push(p),
                    None => continue 'main,
                }
            }
            let mut t1: Vec<String> = vec![String::from("")];
            let mut t2: Vec<String> = Vec::new();
            for lst in poss {
                for v in lst {
                    for old in t1.iter() {
                        let mut new = (*old).clone();
                        new.push(*v);
                        t2.push(new);
                    }
                }
                t1 = t2;
                t2 = Vec::new();
            }
            for p in t1.into_iter() {
                if !cache.contains(&p) {
                    // NOTE: this cache is important
                    stack.push(p.clone());
                    cache.insert(p);
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    //assert!
    println!(
        "{:?}",
        Solution::pyramid_transition(String::from("BCD"), vec!["BCC", "CDE", "CEA", "FFF"])
    );
    assert!(!Solution::pyramid_transition(
        String::from("AAAA"),
        vec!["AAB", "AAC", "BCD", "BBE", "DEF"]
    ));

    println!(
        "{:?}",
        Solution::pyramid_transition(
            String::from("ABBBBA"),
            vec![
                "ACA", "ACF", "ACE", "ACD", "ABA", "ABF", "ABE", "ABD", "FCA", "FCF", "FCE", "FCD",
                "FBA", "FBF", "FBE", "FBD", "ECA", "ECF", "ECE", "ECD", "EBA", "EBF", "EBE", "EBD",
                "DCA", "DCF", "DCE", "DCD", "DBA", "DBF", "DBE", "DBD", "CAA", "CAF", "CAE", "CAD",
                "CFA", "CFF", "CFE", "CFD", "CEA", "CEF", "CEE", "CED", "CDA", "CDF", "CDE", "CDD",
                "BAA", "BAF", "BAE", "BAD", "BFA", "BFF", "BFE", "BFD", "BEA", "BEF", "BEE", "BED",
                "BDA", "BDF", "BDE", "BDD", "CCA", "CCF", "CCE", "CCD", "CBA", "CBF", "CBE", "CBD",
                "BCA", "BCF", "BCE", "BCD", "BBA", "BBF", "BBE", "BBD", "CCC", "CCB", "CBC", "CBB",
                "BCC", "BCB", "BBC", "BBB"
            ]
        )
    );
}
