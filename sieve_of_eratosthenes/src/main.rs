fn main() {
    let bound: u32 = 100;
    let prime_list: Vec<u32> = sieve_of_eratosthenes(bound);

    for prime in prime_list {
        print!("{prime} ");
    }
}

fn sieve_of_eratosthenes(bound: u32) -> Vec<u32> {
    let mut prime_list: Vec<u32> = Vec::new();
    let mut eliminated: Vec<bool> = vec![false; (bound + 1) as usize];

    for i in 2..bound {
        if !eliminated[i as usize] {
            prime_list.push(i);

            let mut j: u32 = i * i;
            while j <= bound {
                eliminated[j as usize] = true;
                j += i;
            }
        }
    }

    prime_list
}
