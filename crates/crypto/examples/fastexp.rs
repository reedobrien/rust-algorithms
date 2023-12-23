use anyhow::Result;

use crypto::{fast_exp, fast_exp_mod, get_number};

fn main() -> Result<()> {
    loop {
        let number = get_number("Enter the number:")?;
        let power = get_number("Enter the power:")?;
        let modulus = get_number("Enter the modulus:")?;

        println!("fast_exp({number}, {power})={}", fast_exp(number, power));
        println!("{number}.pow({power})={}", number.pow(power as u32));

        println!(
            "fast_exp_mod({number}, {power}, {modulus})={}",
            fast_exp_mod(number, power, modulus)
        );
        println!(
            "{number}.pow({power})%{modulus}={}",
            number.pow(power as u32) % modulus
        );
    }
}
