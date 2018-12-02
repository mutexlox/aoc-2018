use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let ints_res : Result<Vec<i32>, _> = input.split_whitespace().map(|s| s.parse::<i32>()).collect();
    let ints = ints_res.unwrap();

    let res : i32 = ints.into_iter().sum();

    println!("{}", res);

    Ok(())
}
