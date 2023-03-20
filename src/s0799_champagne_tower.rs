use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut cache = HashMap::new();
        let n = Solution::rec(poured, query_row, query_glass, &mut cache);
        if n > 1.0 {
            1.0
        } else {
            n
        }
    }

    /// after poured cups, how many cups of liquid flowed through this (row, glass) cup (include the full cup)
    fn rec(
        poured: i32,
        query_row: i32,
        query_glass: i32,
        cache: &mut HashMap<(i32, i32), f64>,
    ) -> f64 {
        if query_glass == 0 || query_glass == query_row {
            if query_row > 30 {
                // poured <= 10^9 < 2 ** 30
                return 0.0;
            }
            let exp = 2i32.pow(query_row as u32);
            let cups_to_start = exp - 1; // 1 + 2 + 4 + ...
            return if poured < cups_to_start {
                0.0
            } else {
                (poured - cups_to_start) as f64 / (exp as f64)
            };
        }
        if let Some(v) = cache.get(&(query_row, query_glass)) {
            return *v;
        }
        let mut flow = 0.0;
        for glass in [query_glass - 1, query_glass] {
            let n = Solution::rec(poured, query_row - 1, glass, cache);
            if n > 1.0 {
                flow += (n - 1.0) / 2.0;
            }
        }
        cache.insert((query_row, query_glass), flow);
        flow
    }
}

#[test]
fn test_0799() {
    assert_eq!(0.0, Solution::champagne_tower(1000000000, 99, 99));
    //dbg!(2i32.pow(99 as u32));
}
