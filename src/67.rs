///2025-11-21
struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut c: Vec<u8> = Vec::new();
        let mut i = a.len() as isize;
        let mut j = b.len() as isize;
        let mut carry: u8 = 0;
        loop {
            let ai = if i > 0 { a[i as usize - 1] - 48 } else { 0 };
            let bi = if j > 0 { b[j as usize - 1] - 48 } else { 0 };
            if i <= 0 && j <= 0 && carry == 0 {
                break;
            }
            let ci = ai + bi + carry;
            c.push(48 + ci % 2);
            carry = ci / 2;
            i -= 1;
            j -= 1;
        }

        c.reverse();
        String::from_utf8(c).unwrap()
    }
}
fn main() {
    let a = "1010".to_string();
    let b = "1011".to_string();
    let result = Solution::add_binary(a, b);
    println!("Result: {:?}", result);
}
