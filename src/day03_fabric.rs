use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Claim {
    id: u32,
    left_margin: u32,
    top_margin: u32,
    width: u32,
    height: u32,
}

#[derive(Debug, Clone)]
struct FormatError;

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid format for input; expect #id @ l,t: wxh")
    }
}

impl error::Error for FormatError {
    fn description(&self) -> &str {
        "invalid format for input; expect #id @ l,t: wxh"
    }
    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl FromStr for Claim {
    type Err = FormatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //#1 @ 257,829: 10x23
        let no_whitespace: String = s.replace(" ", "");
        let pieces: Vec<&str> = no_whitespace
            .trim_matches('#')
            .split(|c| c == '@' || c == ',' || c == ':' || c == 'x')
            .collect();
        let ints_parsed = pieces
            .into_iter()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| FormatError)?;
        //let ints = ints_parsed.unwrap();
        if ints_parsed.len() != 5 {
            Err(FormatError)
        } else {
            Ok(Claim {
                id: ints_parsed[0],
                left_margin: ints_parsed[1],
                top_margin: ints_parsed[2],
                width: ints_parsed[3],
                height: ints_parsed[4],
            })
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let inputs = input.split("\n").map(|s| s.trim()).filter(|s| s.len() != 0);
    let claims = inputs.map(|inp| inp.parse::<Claim>().unwrap());

    let mut claimed = HashMap::<(u32, u32), i32>::new();
    let mut duplicates = 0;
    for c in claims.clone() {
        for i in 0..c.width {
            for j in 0..c.height {
                let i = claimed
                    .entry((c.left_margin + i, c.top_margin + j))
                    .or_insert(0);
                *i += 1;
                if *i == 2 {
                    duplicates += 1;
                }
            }
        }
    }
    println!("{}", duplicates);
    for c in claims {
        let mut overlaps = false;
        for i in 0..c.width {
            for j in 0..c.height {
                let i = claimed.get(&(c.left_margin + i, c.top_margin + j)).unwrap();
                if *i >= 2 {
                    overlaps = true;
                }
            }
        }
        if !overlaps {
            println!("claim ID {} doesn't overlap", c.id);
        }
    }
}
