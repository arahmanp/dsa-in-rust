fn main() {
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let x = 8;

    match binary_search(&arr, x) {
        Some(idx) => println!("the index of {x} is {idx}"),
        None => println!("there is no {x}"),
    }
}

fn binary_search(arr: &[i32], x: i32) -> Option<usize> {
    let mut left: usize = 0;
    let mut right: usize = arr.len() - 1;

    while left <= right {
        let mid: usize = left + (right - left) / 2;

        if arr[mid] < x {
            left = mid + 1;
        } else if arr[mid] > x {
            right = mid - 1;
        } else {
            return Some(mid);
        }
    }

    None
}
