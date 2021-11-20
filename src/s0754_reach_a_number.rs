struct Solution;

impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let mut dist = target.abs();
        let mut step = 0;
        while dist > 0 {
            step += 1;
            dist -= step;
        }
        if (dist & 1) == 0 {
            step
        } else if (step & 1) == 0 {
            step + 1
        } else {
            step + 2
        }
    }
}

#[test]
fn test_754() {
    assert_eq!(1, Solution::reach_number(1));
    assert_eq!(3, Solution::reach_number(2));
    assert_eq!(2, Solution::reach_number(3));
    assert_eq!(44723, Solution::reach_number(-1000000000));
}
