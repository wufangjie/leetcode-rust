use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let dst = dst as usize;
        let mut graph = vec![vec![]; n];
        for triple in flights.into_iter() {
            graph[triple[0] as usize].push((triple[1] as usize, triple[2]));
        }

        let mut heap = BinaryHeap::new();
        for &(t, d) in graph[src as usize].iter() {
            heap.push((-d, t, 0));
        }

        let mut visited = vec![i32::MAX; n]; // min step
        while let Some((cost, s, mut depth)) = heap.pop() {
            if dst == s && depth <= k {
                return -cost;
            }
            if depth < k {
                visited[s] = visited[s].min(depth);
                depth += 1;
                for &(t, d) in graph[s].iter() {
                    if depth < visited[t] {
                        heap.push((cost - d, t, depth));
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn test_0787() {
    assert_eq!(
        700,
        Solution::find_cheapest_price(
            4,
            vec![
                vec![0, 1, 100],
                vec![1, 2, 100],
                vec![2, 0, 100],
                vec![1, 3, 600],
                vec![2, 3, 200],
            ],
            0,
            3,
            1,
        )
    );

    dbg!(Solution::find_cheapest_price(
        5,
        vec![
            vec![0, 1, 5],
            vec![1, 2, 5],
            vec![0, 3, 2],
            vec![3, 1, 2],
            vec![1, 4, 1],
            vec![4, 2, 1]
        ],
        0,
        2,
        2
    ));
}
