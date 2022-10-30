struct Solution;

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
        if sx == tx && sy == ty {
            return true;
        } else if tx < sx || ty < sy {
            return false;
        }

        loop {
            if sx >= ty {
                return ty == sy && ((tx - sx) % ty) == 0;
            } else if sy >= tx {
                return tx == sx && ((ty - sy) % tx) == 0;
            } else if tx > ty {
                tx %= ty;
            } else {
                ty %= tx;
            }
        }
    }
}

#[test]
fn test_780() {
    assert_eq!(true, Solution::reaching_points(1, 1, 3, 5));
    assert_eq!(false, Solution::reaching_points(1, 1, 2, 2));
    assert_eq!(true, Solution::reaching_points(1, 1, 1, 1));

    assert_eq!(true, Solution::reaching_points(1, 2, 999999999, 2));
}
