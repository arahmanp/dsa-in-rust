fn main() {
    let mut arr: Vec<i32> = vec![5, 1, 3, 2, 4];

    println!("before : {:?}", arr);

    insertion_sort(&mut arr);

    println!("after : {:?}", arr);
}

fn insertion_sort(arr: &mut Vec<i32>) {
    let size = arr.len();

    for i in 0..size {
        let mut j = i;

        while j > 0 && arr[j] < arr[j - 1] {
            let tmp = arr[j];
            arr[j] = arr[j - 1];
            arr[j - 1] = tmp;

            j -= 1;
        }
    }
}
