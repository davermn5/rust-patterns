use linearalgorithms::get_window_size;
use linearalgorithms::squared_count_sort;

fn main() {
    let mem_cache: [i32; 3] = [7,7,7];
    let _buff_out = get_window_size(&mem_cache, &21);
    println!("{_buff_out}");

    let mut gpu_n: Vec<i32> = vec![-1,10,3,5,2,2];
    let squared_sorted = squared_count_sort(&mut gpu_n);
    println!("{:?}", squared_sorted);
}