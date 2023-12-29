use anyhow::Result;

use crypto::{fast_exp_mod, find_prime, get_number, inverse_mod, random_exponent, totient};

const NUM_TESTS: usize = 20;
const MIN: usize = 1_000;
const MAX: usize = 10_000;

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();

    // Primes
    let p = find_prime(&mut rng, MIN, MAX, NUM_TESTS);
    let q = find_prime(&mut rng, MIN, MAX, NUM_TESTS);

    // modulus and totient
    let n = p * q;
    let λn = totient(p, q);

    // public (e) and private (d) keys
    let pub_key = random_exponent(&mut rng, λn);
    let priv_key = inverse_mod(pub_key, λn)?;

    println!(
        r#"**** Public ***
Public key Modulus:     {n}
Public key exponenet e: {pub_key}

*** Private ***
Primes:  {p}, {q}
λ(n):    {λn}
d:       {priv_key}
"#
    );

    loop {
        let msg: usize = get_number("Enter a secret number (Use <2 or >10,000 to exit):")?;
        if !(2..MAX).contains(&msg) {
            println!("Bye!");
            break;
        }

        println!("Message:    {msg}");
        let cipher_text = fast_exp_mod(msg, pub_key, n);
        println!("Ciphertext: {cipher_text}");
        let plain_text = fast_exp_mod(cipher_text, priv_key, n);
        println!("Ciphertext: {plain_text}");
        println!();
    }

    Ok(())
}
