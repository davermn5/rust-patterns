// Leetcode
// String and Arrays
// Problem 1: Given a string s, return true if it is a palindrome, false otherwise.
//  Has Time Complexity of O(n) because:
//   - The while loop iterations cost O(1) each
//   - There can never be more than O(n) iterations inside the while loop (because 'i' and 'j' keep moving closer by one step to one another on each iteration).
fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut x = 0;
    let mut y = bytes.len() - 1;

    while x < y {
        if bytes[x] != bytes[y] {
            return false;
        }
        x += 1;
        y -= 1;
    }
    true
}

fn main() {
    let s = "racecar";
    let is_s_palindrome = is_palindrome(&s);
    println!("Is s a Palindrome? {}", is_s_palindrome);
}