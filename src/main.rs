
// Problem: Given an integer array 'nums' sorted in non-decreasing order, return an array of the squares of each number sorted in
//   non-decreasing order.

// Steps to solve:
//  1. For every element 'n' in the array, calculate it's squared value and update in-place
//  2. Bake-in 'count sort' ( Time complexity O(n) )

use std::convert::TryInto;

fn count_sort(arr: &mut Vec<i32>) -> Vec<i32> {

    if arr.is_empty() {
        return arr.to_vec();
    }

    // This is correct solution. It uses a while loop with 'counting sort' to update the values (by squaring them) in-place.
    //   Then, the sort logic below correctly sorts the array. Implemented in (O(n)) time complexity.
    let mut x = 0;
    let y = arr.len() - 1;
    while x <= y {
        let temp = arr[x];
        arr[x] = temp * temp;
        x += 1;
    }

    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    let range: usize = (max - min + 1).try_into().unwrap();

    let mut count: Vec<i32> = vec![0; range];

    for val_ref in arr.iter() {
        count[(*val_ref - min) as usize] += 1;
    }

    let mut sorted: Vec<i32> = Vec::with_capacity(arr.len());

    for (i,c_ref) in count.iter().enumerate() {
        for _ in 0..*c_ref {
            sorted.push((i as i32) + min);
        }
    }
    sorted
}

fn main() {
    let mut arr: Vec<i32> = vec![-4, -1, 0, 3, 10];
    let result = count_sort(&mut arr);
    println!("{:?}", result);
}