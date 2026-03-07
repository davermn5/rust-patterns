//Leetcode
// Arrays and Strings
// Problem 2: Given a sorted array of unique integers and a target integer, return true if there exists
//  a pair of numbers (i.e. "Addends") that sum to target, false otherwise. (This problem is similar to TwoSum; However in Two Sum, the input
//  is not sorted.)
//  Time Complexity of Solution = Linear O(n)
fn sorted_twosums(data: &[i32], target: &i32) -> bool {
    if data.len() < 2 { return false; }

    let mut x = 0;
    let mut y = data.len() - 1;

    while x < y {

        let sum = data[x] + data[y];

        if sum > *target {
            y -= 1;
        }
        else if sum < *target {
            x += 1;
        }
        else {
            return true;
        }
    }// While
    false
}

fn main() {
    let arr_i32: [i32; 6] = [0,1,4,6,9,10];
    let has_two_addends: bool = sorted_twosums(&arr_i32, &5);
    println!("Has two addends? {}", has_two_addends);
}