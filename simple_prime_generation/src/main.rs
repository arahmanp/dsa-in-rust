fn main() {
    let bound: u32 = 100;

    let prime_list: Vec<u32> = simple_prime_generation(bound);

    for prime in &prime_list {
        print!("{prime} ");
    }
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

fn simple_prime_generation(bound: u32) -> Vec<u32> {
    let mut prime_list: Vec<u32> = Vec::new();

    for i in 1..bound {
        if is_prime(i) {
            prime_list.push(i);
        }
    }

    prime_list
}
