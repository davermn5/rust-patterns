
fn linear_search(data: &[i32], needle: &i32) -> Option<usize> {
    for (k, v) in data.iter().enumerate() {
        if v == needle {
            return Some(k);
        }
    }
    None
}

// This function also works
fn linear_search_generic<T: PartialEq>(data: &[T], needle: &T) ->Option<usize> {
    for (k, v) in data.iter().enumerate() {
        if v == needle {
            return Some(k);
        }
    }
    None
}

fn main() {
    let data: [i32; 7] = [0,1,7,9,12,3,4];
    let output = linear_search(&data, &20).unwrap_or(9999); //If end up deciding not to append ".unwrap()", then you can explicitly include the type check (e.g. "output: Option<usize>")
    println!("{:?}", output);

    let mut data2: Vec<i32> = vec![1,2,3];
    data2.push(4);
    let output2 = linear_search(&data2, &3).unwrap_or(9999);
    println!("{:?}", output2);
}