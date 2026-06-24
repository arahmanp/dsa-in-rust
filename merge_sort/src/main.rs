fn main() {
    let mut arr: Vec<i32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];

    println!("before : {:?}", arr);

    merge_sort(&mut arr, 0, 8);

    println!("after : {:?}", arr);
}

fn merge(arr: &mut Vec<i32>, a_left: usize, a_right: usize, b_left: usize, b_right: usize) {
    let size: usize = b_right - a_left + 1;
    let mut tmp: Vec<i32> = Vec::with_capacity(size);
    let mut a_idx: usize = a_left;
    let mut b_idx: usize = b_left;

    while a_idx <= a_right && b_idx <= b_right {
        if arr[a_idx] <= arr[b_idx] {
            tmp.push(arr[a_idx]);
            a_idx += 1;
        } else {
            tmp.push(arr[b_idx]);
            b_idx += 1;
        }
    }

    while a_idx <= a_right {
        tmp.push(arr[a_idx]);
        a_idx += 1;
    }

    while b_idx <= b_right {
        tmp.push(arr[b_idx]);
        b_idx += 1;
    }

    for i in 0..size {
        arr[i + a_left] = tmp[i];
    }
}

fn merge_sort(arr: &mut Vec<i32>, left: usize, right: usize) {
    if left == right {
        return ();
    } else {
        let mid: usize = (left + right) / 2;

        merge_sort(arr, left, mid);
        merge_sort(arr, mid + 1, right);

        merge(arr, left, mid, mid + 1, right);
    }
}
