///2025-11-22
struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = Vec::new();
        let s = s.as_bytes();
        for &i in s.iter() {
            if i == b'(' || i == b'[' || i == b'{' {
                stack.push(i);
                continue;
            }
            if stack.len() == 0 {
                return false;
            }
            let j = stack.pop().unwrap();
            if i - j != 1 && i - j != 2 {
                return false;
            }
        }
        stack.len() == 0
    }
}
fn main() {
    let s = "()".to_string();
    let result = Solution::is_valid(s);
    println!("Result: {:?}", result);
}
