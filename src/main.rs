
// Problem Statement: Given 2 sorted integer arrays arr1 and arr2, return a new array that combines both of them and is also sorted.
// Solution Time Complexity = O(n) | Uses 2 pointers technique.
fn sorted_arrs_combine(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut x1 = 0;
    let y1 = arr1.len() - 1;
    let mut x2 = 0;
    let y2 = arr2.len() - 1;
    let mut out: Vec<i32> = vec![];

    // 'Less than or equal to' iterates through each of the entire arrays
    while x1 <= y1 && x2 <= y2 {
        if arr1[x1] < arr2[x2] {
            out.push(arr1[x1]);
            x1 += 1;
        }
        else {
            if arr2[x2] < arr1[x1] {
                out.push(arr2[x2]);
                x2 += 1;
            }
            else {
                out.push(arr1[x1]);
                x1 += 1;
            }
        }
    }

    while x1 <= y1 {
        out.push(arr1[x1]);
        x1 += 1;
    }

    while x2 <= y2 {
        out.push(arr2[x2]);
        x2 += 1;
    }

    out
}

fn main() {
    let arr1: [i32; 4] = [2,4,4,6];
    let arr2: [i32; 5] = [6,7,8,9,10];

    let result = sorted_arrs_combine(&arr1, &arr2);
    println!("{:?}", result);
}