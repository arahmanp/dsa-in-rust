fn main() {
    let n: u32 = 12;
    let m: u32 = 2_7644_437;

    let factorial: u32 = modular_factorial(n, m);

    println!("factorial of {n} mod {m} is {factorial}");
}

fn modular_factorial(n: u32, m: u32) -> u32 {
    let mut result: u32 = 1;

    for i in 1..n {
        result = (result * (i % m)) % m;
    }

    result
}

