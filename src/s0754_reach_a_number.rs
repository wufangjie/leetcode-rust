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
fn test() {
    println!("{:?}", Solution::reach_number(1));
    println!("{:?}", Solution::reach_number(2));
    println!("{:?}", Solution::reach_number(3));
    println!("{:?}", Solution::reach_number(-1000000000));
}
