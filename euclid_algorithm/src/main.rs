fn main() {
    let a: u32 = 24;
    let b: u32 = 16;

    let gcd: u32 = euclid(a, b);

    println!("{gcd}");
}

fn euclid(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        euclid(b, a % b)
    }
}

