struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut rev = vec![vec![]; n];
        let mut out_degree = Vec::with_capacity(n);
        let mut safe_lst = Vec::with_capacity(n);
        for (i, out_lst) in graph.into_iter().enumerate() {
            out_degree.push(out_lst.len());
            if out_lst.is_empty() {
                safe_lst.push(i as i32)
            }
            for j in out_lst {
                rev[j as usize].push(i);
            }
        }

        let mut un_idx = 0;
        while un_idx < safe_lst.len() {
            for &j in &rev[safe_lst[un_idx] as usize] {
                out_degree[j] -= 1;
                if out_degree[j] == 0 {
                    safe_lst.push(j as i32);
                }
            }
            un_idx += 1;
        }
        safe_lst.sort_unstable();
        safe_lst
    }
}

#[test]
fn test_0802() {
    assert_eq!(
        vec![2, 4, 5, 6],
        Solution::eventual_safe_nodes(vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![]
        ])
    )
}
