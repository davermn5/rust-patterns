use std::convert::TryInto;
use std::cmp::max;

/*
 Algorithm: Squared Count Sort
    Provided an unsorted Vec of signed integers 32bits in length, square each element's value before returning.
    Time Complexity: (O(n + k)) with (O(n))
    Space Complexity: (O(k)) => (O(1))
 */

fn squared_count_sort(arr: &mut Vec<i32>) -> Vec<i32> {

    if arr.is_empty() {
        return arr.to_vec();
    }

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

/*
 Algorithm: The Moving Window
    Provided an input array 'n' and a target sum, return the maximum size of the array's subset (the window)
    whose element values add up to the input 'target_sum' as the iterator traverses 'n'.
    Time Complexity: (O(n))
 */
fn get_window_size(n: &[i32], target_sum: &i32) -> i32 {
    if n.is_empty() {
        return 0;
    }

    let mut left = 0;
    // 'right' will be defined below
    let mut curr_sum = 0;
    let mut window_size = 0;

    for (right,_) in n.iter().enumerate() {
        curr_sum += n[right];

        while curr_sum > *target_sum {
            curr_sum -= n[left];
            left += 1;
        }
        window_size = max(window_size, (right as i32) - (left as i32) + 1);
    }
    return window_size.try_into().unwrap();
}

//UNIT TESTS
#[cfg(test)]
mod linear_algorithms {
    use super::*;

    #[test]
    fn test_squared_count_sort() {
        let mut arr: Vec<i32> = vec![-4, -1, 0, 3, 10];
        let result = squared_count_sort(&mut arr);
        assert_eq!([0, 1, 9, 16, 100], *result);
    }

    #[test]
    fn test_get_window_size() {
        let n: [i32; 3] = [7,7,7];
        let _buff_out = get_window_size(&n, &21);
        assert_eq!(3, _buff_out);
    }
}