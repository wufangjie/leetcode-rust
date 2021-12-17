use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    fn get_list(
        // for way 2
        bottom: &String,
        i: usize,
        lst: &mut Vec<char>,
        ret: &mut Vec<String>,
        dct: &HashMap<&str, Vec<char>>,
    ) {
        if i + 1 == bottom.len() {
            ret.push(lst.iter().collect::<String>());
        } else {
            for ch in dct.get(&bottom[i..i + 2]).as_ref().unwrap().iter() {
                lst.push(*ch);
                Self::get_list(bottom, i + 1, lst, ret, dct);
                lst.pop();
            }
        }
    }

    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut allowed_map: HashMap<&str, Vec<char>> = HashMap::new();
        for s in allowed.iter() {
            allowed_map
                .entry(&s[..2])
                .or_insert(vec![])
                .push((&s[2..3]).parse().unwrap());
        }

        let mut cache: HashSet<String> = HashSet::new();
        let mut stack = vec![bottom];
        'main: while let Some(bottom) = stack.pop() {
            let mut poss: Vec<&Vec<char>> = vec![];
            for i in 0..(bottom.len() - 1) {
                match allowed_map.get(&bottom[i..i + 2]) {
                    Some(p) => poss.push(p),
                    None => continue 'main,
                }
            }
            if bottom.len() == 2 {
                return true;
            }

            // let mut t1 = vec![];
            // Self::get_list(&bottom, 0, &mut vec![], &mut t1, &allowed_map);

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
                // t1 = t2;
                // t2 = vec![];
                std::mem::swap(&mut t1, &mut t2);
                t2.clear();
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
fn test_756() {
    assert!(Solution::pyramid_transition(
        String::from("BCD"),
        vec!["BCC", "CDE", "CEA", "FFF"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>(),
    ));
    assert!(!Solution::pyramid_transition(
        String::from("AAAA"),
        vec!["AAB", "AAC", "BCD", "BBE", "DEF"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>(),
    ));

    assert!(!Solution::pyramid_transition(
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
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>(),
    ));
}
