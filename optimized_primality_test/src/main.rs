fn main() {
    let num: u32 = 12_333_113;
    let status: bool = is_prime_opt(num);

    println!("is {num} a prime? {status}");
}

fn is_prime_opt(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    let mut i: u32 = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}

