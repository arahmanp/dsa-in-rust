fn main() {
    let mut arr: Vec<i32> = vec![5, 4, 3, 2, 1, 8, 8, 8, 6];

    println!("before : {:?}", arr);

    counting_sort(&mut arr);

    println!("after : {:?}", arr);
}

fn counting_sort(arr: &mut Vec<i32>) {
    let mut freq: [u32; 100_001] = [0; 100_001];

    for i in arr.iter() {
        freq[(*i) as usize] += 1;
    }

    let mut idx: usize = 0;
    for i in 0..100_001 {
        for _j in 0..freq[i] {
            arr[idx] = i as i32;
            idx += 1;
        }
    }
}
