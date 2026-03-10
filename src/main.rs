
//Problem: Given an input array of chars (the visual string), return the same array holding the values stored in reverse order.
//Solution Time Complexity: O(n)  linear
fn reverse_string(s: &mut Vec<char>) -> &mut Vec<char> {
    let mut x = 0;
    let mut y = s.len() - 1;

    while x < y {
        let temp = s[x];
        s[x] = s[y];
        s[y] = temp;
        x += 1;
        y -= 1;
    }
    s
}

fn main() {
    let mut s: Vec<char> = vec!['e','x','p','e','r','t'];
    let result = reverse_string(&mut s);

    println!("{:?}", result);
}