
// Problem: Given an integer array 'nums' sorted in non-decreasing order, return an array of the squares of each number sorted in
//   non-decreasing order.

// Steps to solve:
//  1. For every element 'n' in the array, calculate it's squared value and update in-place
//  2. Bake-in 'count sort' ( Time complexity O(n) )

fn count_sort(num: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut x = 0;
    let y = num.len() - 1;

    while x <= y {
        let temp = num[x];
        num[x] = temp * temp;
        x += 1;
    }
    num
}

fn main() {
    let mut num: Vec<i32> = vec![-4,-1,0,3,10];
    let result = count_sort(&mut num);
    println!("{:?}", result);
}