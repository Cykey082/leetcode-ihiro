///2025-11-23
struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup(); //Sorry I was looking for "de"lete
        nums.len() as i32
    }
}
fn main() {
    let mut nums = vec![1, 1, 2];
    let result = Solution::remove_duplicates(&mut nums);
    println!("Result: {:?}, nums = {:?}", result, nums);
}
