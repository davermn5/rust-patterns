
//Problem: Given 2 string slice inputs (s, t), return true if s can be found in the same order as it appears in t, otherwise false.
// For example: "ace" can be found in "abcde", but not "acer". Validate bool success once the iterator has crossed the finish-line of s_bytes.len().
fn is_subsequence(s: &str, t: &str) -> bool {
    let s_bytes = s.as_bytes();
    let mut x1 = 0;
    let y1 = s_bytes.len() - 1;

    let t_bytes = t.as_bytes();
    let mut x2 = 0;
    let y2 = t_bytes.len() - 1;

    while x1 <= y1 && x2 <= y2 {
        if s_bytes[x1] == t_bytes[x2] {
            x1 += 1;
        }
        x2 += 1;
    }
    x1 == y1 + 1
}

fn main() {
    let s = "ace";
    let t = "abcde";

    let result = is_subsequence(&s, &t);
    println!("Is s a Subsequence of t? {}", result);
}