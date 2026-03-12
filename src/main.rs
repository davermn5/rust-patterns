
// Problem: Given an integer array 'nums' sorted in non-decreasing order, return an array of the squares of each number sorted in
//   non-decreasing order.

fn create_sorted(num: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut x = 0;
    let mut y = 1;

    while x < num.len() - 1 {
        if num[y] >= num[x] {
            x += 1;
            y += 1
        }
        else {                        //num[y] < num[x]
            let temp = num[y];
            num[y] = num[x];
            num[x] = temp;
            x += 1;
            y += 1;
        }
    }
    num
}

fn main() {
    let mut num: Vec<i32> = vec![16, 1, 0, 9, 100];
    let result = create_sorted(&mut num);
    println!("{:?}", result);
}