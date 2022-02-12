struct Solution;

// impl Solution {
//     pub fn can_transform(start: String, end: String) -> bool {
//         if start.len() != end.len() {
//             false
//         } else {
//             let mut state = 0;
//             // 0 means previous same,
//             // - means we need XL sequence (but more L in start)
//             // + means we need RX sequence (but more X in start)
//             for i in 0..start.len() {
//                 match state {
//                     0 => {
//                         if &start[i..i + 1] != &end[i..i + 1] {
//                             match &end[i..i + 1] {
//                                 "L" => state = -1,
//                                 "X" => state = 1,
//                                 _ => return false,
//                             }
//                         }
//                     }
//                     x if x < 0 => {
//                         if &start[i..i + 1] == "R" || &end[i..i + 1] == "R" {
//                             return false;
//                         } else if &start[i..i + 1] == &end[i..i + 1] {
//                             continue;
//                         } else if &start[i..i + 1] == "L" {
//                             state += 1;
//                         } else {
//                             state -= 1;
//                         }
//                     }
//                     _ => {
//                         if &start[i..i + 1] == "L" || &end[i..i + 1] == "L" {
//                             return false;
//                         } else if &start[i..i + 1] == &end[i..i + 1] {
//                             continue;
//                         } else if &start[i..i + 1] == "R" {
//                             state += 1;
//                         } else {
//                             state -= 1;
//                         }
//                     }
//                 }
//             }
//             state == 0
//         }
//     }
// }

impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        if start.len() != end.len() {
            false
        } else {
            let mut state = 0;
            // 0 means previous same,
            // - means we need XL sequence (but more L in start)
            // + means we need RX sequence (but more X in start)
            for (cs, ce) in start.bytes().zip(end.bytes()) {
                match state {
                    0 => {
                        if cs != ce {
                            match ce {
                                b'L' => state = -1,
                                b'X' => state = 1,
                                _ => return false,
                            }
                        }
                    }
                    x if x < 0 => {
                        if cs == b'R' || ce == b'R' {
                            return false;
                        } else if cs == ce {
                            continue;
                        } else if cs == b'L' {
                            state += 1;
                        } else {
                            state -= 1;
                        }
                    }
                    _ => {
                        if cs == b'L' || ce == b'L' {
                            return false;
                        } else if cs == ce {
                            continue;
                        } else if cs == b'R' {
                            state += 1;
                        } else {
                            state -= 1;
                        }
                    }
                }
            }
            state == 0
        }
    }
}

#[test]
fn test_777() {
    assert!(Solution::can_transform(
        "RXXLRXRXL".to_owned(),
        "XRLXXRRLX".to_owned()
    ));
    assert!(!Solution::can_transform("X".to_owned(), "L".to_owned()));

    assert!(!Solution::can_transform(
        "XRRXRX".to_owned(),
        "RXLRRX".to_owned()
    ));

    // NOTE: only RX -> XR, XL -> LX
    assert!(!Solution::can_transform(
        "LXXLXRLXXL".to_owned(),
        "XLLXRXLXLX".to_owned()
    ));

    assert!(Solution::can_transform(
        "XXRXLXRXXX".to_owned(),
        "XXRLXXXXXR".to_owned()
    ));

    assert!(Solution::can_transform(
        "XLXRRXXRXX".to_owned(),
        "LXXXXXXRRR".to_owned()
    ));

    // NOTE: it's really hard to consider every possible cases
}
