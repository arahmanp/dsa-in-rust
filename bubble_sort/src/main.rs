fn main() {
    let mut arr: Vec<i32> = vec![5, 2, 3, 4, 1];

    println!("before : {:?}", arr);

    bubble_sort(&mut arr);

    println!("after : {:?}", arr);
}

fn bubble_sort(arr: &mut Vec<i32>) {
    let size = arr.len();

    for i in 0..(size - 1) {
        for j in 0..(size - i - 1) {
            if arr[j] > arr[j + 1] {
                let tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
            }
        }
    }
}
