fn main() {
    let mut arr: Vec<i32> = vec![5, 4, 3, 2, 1];

    println!("before : {:?}", arr);

    selection_sort(&mut arr);

    println!("after : {:?}", arr);
}

fn selection_sort(arr: &mut Vec<i32>) {
    let size = arr.len();

    for i in 0..(size - 1) {
        let mut min_idx = i;

        for j in i..size {
            if arr[min_idx] > arr[j] {
                min_idx = j;
            }
        }

        let tmp = arr[i];
        arr[i] = arr[min_idx];
        arr[min_idx] = tmp;
    }
}
