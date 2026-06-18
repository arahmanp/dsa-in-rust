fn main() {
    let num: u32 = 21_423_883;
    let status: bool = is_prime(num);

    println!("is {num} a prime? {status}");
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..(n - 1) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

