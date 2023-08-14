impl Solution {
    /// we can always left an odd bit, so that Bob can never get a zero
    pub fn xor_game(nums: Vec<i32>) -> bool {
	let all = nums.iter().fold(0, |acc, x| acc ^ x);
	all == 0 || nums.len() & 1 == 1 // if condtion is slower
    }
}
