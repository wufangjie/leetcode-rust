impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let shortest = target[0].abs() + target[1].abs();
        for pair in ghosts {
            if (target[0] - pair[0]).abs() + (target[1] - pair[1]).abs() <= shortest {
                return false;
            }
        }
        true
    }
}
