fn main() {
    let arr: Vec<i32> = vec![1, 2, 3, 4, 44, 5, 2, 9];
    let x: i32 = 10;

    match linear_search(&arr, x) {
        Some(idx) => println!("{idx}"),
        None => println!("{x} doesn't exist")
    }
}

fn linear_search(arr: &Vec<i32>, x: i32) -> Option<usize> {
    let size = arr.len();

    for i in 0..size {
        if arr[i] == x {
            return Some(i);
        }
    }

    None
}
