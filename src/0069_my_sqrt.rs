///2025-11-21
struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let mut x0 = x / 2;
        loop {
            let x1 = (x0 + x / x0) / 2;
            if x1 >= x0 {
                //which means iter makes no more progress
                return x0;
            }
            x0 = x1;
        }
    }
}
fn main() {
    let x = 0x7fffffff;
    let result = Solution::my_sqrt(x);
    println!("Result: {:?}", result);
}
