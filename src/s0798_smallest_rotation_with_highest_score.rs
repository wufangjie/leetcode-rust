struct Solution;

impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut delta = vec![0; n + 1]; // inc/dec points
        for (i, &v) in nums.iter().enumerate() {
            if v != 0 {
                // 0 is ok to put everywhere
                let v = v as usize;
                if v <= i {
                    delta[0] += 1;
                    delta[i - v + 1] -= 1;
                    delta[i + 1] += 1; // 这里最开始没注意写成了等号
                } else {
                    delta[i + 1] += 1;
                    delta[i + n - v + 1] -= 1;
                }
            }
        }
        let mut res = 0;
        let mut cur = delta[0];
        let mut best = cur;
        for (i, d) in delta.into_iter().enumerate().take(n).skip(1) {
            cur += d;
            if cur > best {
                res = i;
                best = cur;
            }
        }
        res as i32
    }
}


impl Solution {
    pub fn best_rotation_raw(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut inc = vec![0; n + 1]; // get point
        let mut dec = vec![0; n + 1]; // lose point
        for (i, &v) in nums.iter().enumerate() {
            if v != 0 {
                // 0 is ok to put everywhere
                let v = v as usize;
                if v <= i {
                    inc[0] += 1;
                    dec[i - v + 1] -= 1;
                    inc[i + 1] = 1;
                } else {
                    inc[i + 1] += 1;
                    dec[i + n - v + 1] -= 1;
                }
            }
        }
        let mut res = 0;
        let mut cur = inc[0];
        let mut best = cur;
        for i in 1..n {
            cur += inc[i] + dec[i];
            if cur > best {
                res = i;
                best = cur;
            }
        }
        res as i32
    }
}

// TLE solution
// impl Solution {
//     pub fn best_rotation(nums: Vec<i32>) -> i32 {
//         let n = nums.len();
//         let mut stack1 = Vec::with_capacity(n); // can get point
//         let mut stack2 = Vec::with_capacity(n); // poss to get point
//         for (i, &v) in nums.iter().enumerate() {
//             if v != 0 {
//                 // 0 is ok to put everywhere
//                 let i = i as i32;
//                 if v <= i {
//                     stack1.push((0, i - v));
//                 }
//                 stack2.push((i + 1, i + n as i32 - v));
//             }
//         }
//         stack1.sort_unstable_by_key(|&x| -x.1);
//         stack2.sort_unstable_by_key(|&x| -x.0);

//         let mut res = 0;
//         let mut count = stack1.len();
//         while let Some(span) = stack2.pop() {
//             Solution::binary_insert_1(&mut stack1, span);
//             while let Some(span2) = stack2.last() {
//                 if span.0 == span2.0 {
//                     Solution::binary_insert_1(&mut stack1, stack2.pop().unwrap());
//                 } else {
//                     break;
//                 }
//             }
//             while let Some(span3) = stack1.last() {
//                 if span3.1 < span.0 {
//                     stack1.pop().unwrap();
//                 } else {
//                     break;
//                 }
//             }
//             if stack1.len() > count {
//                 count = stack1.len();
//                 res = span.0;
//             }
//         }
//         res
//     }

//     fn binary_insert_1(arr: &mut Vec<(i32, i32)>, pair: (i32, i32)) {
//         let i = match arr.binary_search_by_key(&-pair.1, |x| -x.1) {
//             Err(i) => i,
//             Ok(i) => i,
//         };
//         arr.insert(i, pair)
//     }

//     fn binary_insert_0(arr: &mut Vec<(i32, i32)>, pair: (i32, i32)) {
//         let i = match arr.binary_search_by_key(&-pair.0, |x| -x.0) {
//             Err(i) => i,
//             Ok(i) => i,
//         };
//         arr.insert(i, pair)
//     }
// }

#[test]
fn test_0798() {
    dbg!(Solution::best_rotation(vec![2, 3, 1, 4, 0]));
    dbg!(Solution::best_rotation(vec![1, 3, 0, 2, 4]));
    dbg!(Solution::best_rotation(vec![77; 100000]));
}
