struct Solution;

const TYPE: [usize; 10] = [1, 1, 2, 0, 0, 2, 2, 0, 1, 2];
const COUNT_NOT_SYM: [i32; 10] = [1, 2, 3, 3, 3, 4, 5, 5, 6, 7];
const COUNT_SYM: [i32; 10] = [0, 0, 1, 1, 1, 2, 3, 3, 3, 4];

// ambiguous: each character rotate 180 or the whole digits rotate 180
// turn out to be the former

impl Solution {
    fn brute_force_one(mut n: i32) -> bool {
        let mut count = 0;
        while n > 0 {
            let v = n % 10;
            match TYPE[v as usize] {
                0 => return false,
                2 => {
                    count += 1;
                }
                _ => (),
            }
            n /= 10;
        }
        count > 0
    }

    fn brute_force(n: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(n as usize + 1);
        let mut cnt = 0;
        res.push(0);
        for i in 1..n {
            cnt += Self::brute_force_one(i) as i32;
            res.push(cnt);
        }
        res
    }

    pub fn rotated_digits(mut n: i32) -> i32 {
        let mut digits = Vec::with_capacity(5); // <= 10e4
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        let mut res = 0;
        let mut prs = true; // possible rotated symmetric
        for left in (1..digits.len() as u32).rev() {
            let d = digits.pop().unwrap() as usize;
            let mut count = [0; 3];
            for i in 0..d {
                count[TYPE[i]] += 1;
            }

	    // leading 0s will not influence the result
            res += (count[1] + count[2]) * 7i32.pow(left);
            if prs {
                res -= count[1] * 3i32.pow(left);
                if TYPE[d] == 2 {
                    prs = false;
                }
            }
            if TYPE[d] == 0 {
                return res;
            }
        }

        let d = digits[0] as usize;
        res + if prs { COUNT_SYM[d] } else { COUNT_NOT_SYM[d] }
    }
}

#[test]
fn test_0788() {
    let n = 1000;
    let res = Solution::brute_force(n);
    // //dbg!(&res);
    // dbg!(res[10]);
    // dbg!(res[459]);
    // dbg!(res[857]);
    for i in 1..n {
        let r = Solution::rotated_digits(i);
        if res[i as usize] != r {
            println!("{i}: expected: {}, got: {r}", res[i as usize]);
            break;
        }
    }

    assert_eq!(4, Solution::rotated_digits(10));
    assert_eq!(0, Solution::rotated_digits(1));
    assert_eq!(1, Solution::rotated_digits(2));
    assert_eq!(247, Solution::rotated_digits(857));
    assert_eq!(129, Solution::rotated_digits(459));
}
