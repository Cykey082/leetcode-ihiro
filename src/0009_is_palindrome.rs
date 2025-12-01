///2025-11-21
struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x <= 9 {
            return true;
        }
        let mut xs: Vec<i32> = Vec::new();
        let mut x = x;
        while x != 0 {
            xs.push(x % 10);
            x /= 10;
        }
        println!("{:?}", xs);
        let mut l = 0;
        let mut r = xs.len() - 1;
        while l <= r {
            if xs[l] != xs[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
fn main() {
    let x = 121;
    let result = Solution::is_palindrome(x);
    println!("Result: {:?}", result);
}
