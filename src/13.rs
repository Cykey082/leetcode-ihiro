///2025-11-21
struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.as_bytes();
        let mut sum = 0;
        let mut i = s.len();
        let mut max: i32 = 0;
        while i >= 1 {
            let val = match s[i - 1] {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => 0,
            };
            if val >= max {
                max = val;
                sum += val;
            } else {
                sum -= val;
            }
            i -= 1;
        }
        sum
    }
}
fn main() {
    let s = "MCDXXXVII".to_string();
    let result = Solution::roman_to_int(s);
    println!("Result: {:?}", result);
}
