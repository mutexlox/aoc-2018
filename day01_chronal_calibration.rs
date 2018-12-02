use std::io::{self, Read};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let ints_res : Result<Vec<i32>, _> = input.split_whitespace().map(|s| s.parse::<i32>()).collect();
    let mut ints = ints_res.unwrap().into_iter().cycle();

    let mut sum = 0;
    let mut seen = HashSet::<i32>::new();
    while !seen.contains(&sum) {
        seen.insert(sum);
        sum += ints.next().unwrap();
    }

    println!("{}", sum);

    Ok(())
}
