struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let n = graph.len();
        let target = n as i32 - 1;
        let mut stack: Vec<(i32, Vec<Vec<i32>>)> = vec![(0, vec![vec![]])];
        while let Some((last, mut pres)) = stack.pop() {
            for pre in &mut pres {
                pre.push(last);
            }
            if last == target {
                res.extend(pres);
            } else {
                let outs = &graph[last as usize];
                if !outs.is_empty() {
                    for &i in outs.iter().skip(1) {
                        stack.push((i, pres.clone()));
                    }
                    stack.push((outs[0], pres));
                }
            }
        }
        res
    }
}

// rec version
// impl Solution {
//     pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         let n = graph.len();
//         let mut res = Self::rec(0, n as i32 - 1, &graph);
//         for path in &mut res {
//             path.reverse();
//         }
//         res
//     }

//     fn rec(i: i32, target: i32, graph: &[Vec<i32>]) -> Vec<Vec<i32>> {
//         if i == target {
//             vec![vec![i]]
//         } else {
//             let mut res = vec![];
//             for &j in &graph[i as usize] {
//                 for mut post in Self::rec(j, target, graph) {
//                     post.push(i);
//                     res.push(post);
//                 }
//             }
//             res
//         }
//     }
// }

#[test]
fn test_0797() {
    dbg!(Solution::all_paths_source_target(vec![
        vec![4, 3, 1],
        vec![3, 2, 4],
        vec![3],
        vec![4],
        vec![]
    ]));
}
