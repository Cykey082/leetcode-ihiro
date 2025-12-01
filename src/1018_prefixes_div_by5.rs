///2025-11-24
struct Solution;
impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result: Vec<bool> = Vec::new();
        let mut num = 0;
        for i in nums {
            num = num * 2 % 10 + i;
            result.push(num % 5 == 0);
        }
        result
    }
}
fn main() {
    let nums = vec![1, 0, 1];
    let result = Solution::prefixes_div_by5(nums);
    println!("Result: {:?}", result);
}
