impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let n = s.len();
        if goal.len() != n {
            return false;
        }
        for i in 0..n - 1 {
            if s.starts_with(&goal[i..]) && goal.starts_with(&s[n-i..]) {
                return true;
            }
        }
        false
    }
}
