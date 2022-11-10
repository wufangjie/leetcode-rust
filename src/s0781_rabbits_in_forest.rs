struct Solution;

// NOTE: 题目可能不太好理解, 就是这 n 个是不同的兔子

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut seen = std::collections::HashMap::with_capacity(answers.len());
        for i in answers {
            *seen.entry(i).or_insert(0) += 1;
        }
        seen.into_iter()
            .fold(0, |accum, (k, v)| accum + (v + k) / (k + 1) * (k + 1))
    }

    // pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    //     let mut seen = std::collections::HashMap::with_capacity(answers.len());
    //     let mut res = 0;
    //     for i in answers {
    //         let old = seen.entry(i).or_insert(0);
    //         if *old % (i + 1) == 0 {
    //             res += i + 1;
    //         }
    //         *old += 1;
    //     }
    //     res
    // }
}

#[test]
fn test_781() {
    assert_eq!(5, Solution::num_rabbits(vec![1, 1, 2]));
    assert_eq!(11, Solution::num_rabbits(vec![10, 10, 10]));
    assert_eq!(5, Solution::num_rabbits(vec![1, 0, 1, 0, 0]));
    assert_eq!(6, Solution::num_rabbits(vec![0, 0, 1, 1, 1]));
}
