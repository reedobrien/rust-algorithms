use anyhow::{anyhow, Result};
use sorting::{get_count, make_one};
// https://liveproject.manning.com/module/1552_5_1/sorting-and-searching-rust/4--linear-search/4-1-workflow%3a-linear-search

fn main() -> Result<()> {
    let size = count_prompt()?;
    let max = get_count("Max:")?;
    let v = make_one(size, max);

    loop {
        let query = get_count::<i32>("Target (-1 to quit):")?;
        match query {
            1..=40 => {
                let res = linear_search(&v, query);
                if res.0 != -1 {
                    println!("numbers[{}] = {}, {} tests", res.0, query, res.1);
                } else {
                    println!("Target {} not found, {} tests", query, res.1);
                }
            }
            -1 => std::process::exit(0),
            _ => {
                eprintln!("Ivalid input...");
                std::process::exit(-1);
            }
        }
    }
}

fn count_prompt() -> Result<usize> {
    let cnt = get_count("Items: ")?;
    match cnt {
        1..=40 => Ok(cnt),
        _ => Err(anyhow!("The input {cnt} should be from 1-40 inclusive")),
    }
}

fn linear_search(v: &Vec<i32>, q: i32) -> (i32, i32) {
    for (i, e) in v.iter().enumerate() {
        if *e == q {
            return (i as i32, i as i32 + 1);
        }
    }
    (-1, v.len() as i32)
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn test_linear_search() {
        let v = vec![
            22, 10, 52, 9, 78, 76, 41, 36, 99, 23, 1, 74, 38, 12, 44, 52, 67, 14, 1, 58,
        ];

        assert_eq!(linear_search(&v, 1), (10, 11));
        assert_eq!(linear_search(&v, 99), (8, 9));
        assert_eq!(linear_search(&v, -1), (-1, 20));
    }
}
