struct Solution;

impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut arr = Vec::new();
        for pair in intervals.into_iter() {
            arr.push((pair[0], pair[1]));
        }
        arr.sort_by(|a, b| (a.1, -a.0).cmp(&(b.1, -b.0)));

        let n = arr.len();
        let mut idx: Vec<usize> = (0..n).into_iter().collect();
        idx.sort_by(|a, b| arr[*a].0.cmp(&arr[*b].0));
        arr.push((1e9 as i32, 0));
        idx.push(n);

        let mut count = vec![2i8; n];
        let mut ret = 0;
        let mut j = 0;
        let mut k = -1; // min unfinished lower bound
        for i in 0..n {
            if arr[i].0 < k || count[i] < 1 {
                continue;
            }
            ret += count[i] as i32;
            k = arr[idx[j]].0;
            if count[i] == 2 {
                while arr[idx[j]].0 < arr[i].1 {
                    count[idx[j]] = 0;
                    j += 1;
                }
                while arr[idx[j]].0 == arr[i].1 {
                    count[idx[j]] = 1;
                    j += 1;
                }
            } else {
                while arr[idx[j]].0 <= arr[i].1 {
                    count[idx[j]] -= 1;
                    j += 1;
                }
            };
        }
        ret
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::intersection_size_two(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]]),
        3
    );
    assert_eq!(
        Solution::intersection_size_two(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]]),
        5
    );

    assert_eq!(
        Solution::intersection_size_two(vec![
            vec![2, 10],
            vec![3, 7],
            vec![3, 15],
            vec![4, 11],
            vec![6, 12],
            vec![6, 16],
            vec![7, 8],
            vec![7, 11],
            vec![7, 15],
            vec![11, 12]
        ]),
        5
    );

    assert_eq!(
        Solution::intersection_size_two(vec![vec![0, 2], vec![2, 3], vec![1, 3]]),
        3
    );

    assert_eq!(
        Solution::intersection_size_two(vec![
            vec![33, 44],
            vec![42, 43],
            vec![13, 37],
            vec![24, 33],
            vec![24, 33],
            vec![25, 48],
            vec![10, 47],
            vec![18, 24],
            vec![29, 37],
            vec![7, 34]
        ]),
        6
    );
}
