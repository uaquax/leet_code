struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.to_string() == x.to_string().chars().rev().collect::<String>() {
            return true;
        }

        false
    }
}

fn main() {
    let result = Solution::is_palindrome(121);
    println!("Result: {:?}", result);
}
