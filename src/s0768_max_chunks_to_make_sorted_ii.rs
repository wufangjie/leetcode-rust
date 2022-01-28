struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut can_be_end = vec![n - 1];

        let mut the_min = arr[n - 1];
        for i in (0..n - 1).into_iter().rev() {
            if arr[i] <= the_min {
                can_be_end.push(i);
                the_min = arr[i];
            }
        }
        can_be_end.reverse();

        let mut count = 1;
        let mut the_max = arr[0];
        let mut start_i = 1;
        let mut i = 0;

        while i < can_be_end.len() - 1 {
            the_max = Self::find_max(&arr, start_i, can_be_end[i], the_max);
            start_i = can_be_end[i] + 1;
            if the_max <= arr[can_be_end[i + 1]] {
                count += 1;
            }
            i += 1;
        }
        count
    }

    fn find_max(arr: &[i32], lo: usize, hi: usize, mut the_max: i32) -> i32 {
        for i in lo..=hi {
            if arr[i] > the_max {
                the_max = arr[i];
            }
        }
        the_max
    }
}

#[test]
fn test_768() {
    assert_eq!(1, Solution::max_chunks_to_sorted(vec![5, 4, 3, 2, 1]));
    assert_eq!(4, Solution::max_chunks_to_sorted(vec![2, 1, 3, 4, 4]));
    assert_eq!(2, Solution::max_chunks_to_sorted(vec![0, 3, 0, 3, 2]))
}
