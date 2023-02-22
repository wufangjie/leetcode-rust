impl Solution {
    /// case1: 2/7*5/7*(2 + 3*2/7 + 4*(2/7)**2 + ...)
    /// case2: 5/7*6/7*(2 + 3*1/7 + 4*(1/7)**2 + ...)
    /// solve it, we can expect total cost: 10/49*84/25+30/49*91/36=2.233333
    pub fn rand10() -> i32 {
        let r1 = rand7();
        if r1 > 5 {
            loop {
                let r2 = rand7();
                if r2 < 6 {
                    return r2 * 2 - if r1 == 6 { 0 } else { 1 };
                }
            }
        } else {
            loop {
                let r2 = rand7();
                if r2 != 7 {
                    return r1 * 2 - r2 + ((r2 >> 1) << 1)
                }
            }
        }
    }
}
