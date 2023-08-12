struct Solution;

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut cur = 0;
        let mut rows = 1;
        for c in s.bytes() {
            let i = (c - b'a') as usize;
            if cur + widths[i] > 100 {
                cur = widths[i];
                rows += 1;
            } else {
                cur += widths[i];
            }
        }
        vec![rows, cur]
    }
}

#[test]
fn test_0806() {
    assert_eq!(
        vec![3, 60],
        Solution::number_of_lines(
            vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        )
    );

    assert_eq!(
        vec![2, 4],
        Solution::number_of_lines(
            vec![
                4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            "bbbcccdddaaa".to_string()
        )
    );
}
