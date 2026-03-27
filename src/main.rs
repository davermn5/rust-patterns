/*
 Problem:  Moving window
 Given an input array 'n', get the max size of the array subset (the window),
  whose elements add up to the input 'target_sum'.
 */

use std::cmp::max;
use std::convert::TryInto;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

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
fn main() {
    let n: [i32; 3] = [7,7,7];
    let _buff_out = get_window_size(&n, &21);
    println!("{_buff_out}");
}