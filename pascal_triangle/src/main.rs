fn main() {
    let n: usize = 6;

    let c = pascal_triangle(n);

    let some_num = c[6][3];

    println!("{some_num}");
}

fn pascal_triangle(n: usize) -> Vec<Vec<u32>> {
    let mut c: Vec<Vec<u32>> = vec![vec![0; n + 1]; n + 1];

    for i in 0..(n + 1) {
        c[i][0] = 1;

        for j in 1..i {
            c[i][j] = c[i - 1][j] + c[i - 1][j - 1];
        }

        c[i][i] = 1;
    }

    c
}

