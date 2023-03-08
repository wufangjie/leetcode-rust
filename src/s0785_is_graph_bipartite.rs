struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut which_part = vec![0u8; n];
        let mut visited = vec![0u8; n]; // 0 unvisited, 1 visited but not popped, 2 popped
        let mut stack = Vec::with_capacity(n / 3);
        for i in 0..n {
            if visited[i] < 2 {
                stack.push(i);
                while let Some(j) = stack.pop() {
                    if visited[j] < 2 {
                        visited[j] = 2;
                        let part = which_part[j] ^ 1;
                        for &k in &graph[j] {
                            let k = k as usize;
                            if visited[k] == 0 {
                                which_part[k] = part;
                                visited[k] = 1;
                                stack.push(k);
                            } else if which_part[k] != part {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
}

#[test]
fn test_0785() {
    assert!(!Solution::is_bipartite(vec![
        vec![1, 2, 3],
        vec![0, 2],
        vec![0, 1, 3],
        vec![0, 2]
    ]));
    assert!(Solution::is_bipartite(vec![
        vec![1, 3],
        vec![0, 2],
        vec![1, 3],
        vec![0, 2]
    ]))
}
