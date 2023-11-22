fn main() {
    for n in 0..22 {
        println!("{n}! = {}", factorial(n));
    }
}

fn factorial(n: u128) -> u128 {
    if n == 0 {
        return 1;
    }

    n * factorial(n - 1)
}
