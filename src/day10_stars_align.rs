extern crate regex;

use regex::Regex;
use std::error;
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Pair {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Point {
    position: Pair,
    velocity: Pair,
}

#[derive(Debug, Clone)]
struct FormatError;

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid format for input; expect 'position=< 9,  1> velocity=< 0,  2>'"
        )
    }
}

impl error::Error for FormatError {
    fn description(&self) -> &str {
        "invalid format for input; expect 'position=< 9,  1> velocity=< 0,  2>'"
    }
    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl FromStr for Point {
    type Err = FormatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"position=<\s*(-?\d*),\s*(-?\d*)>\s*velocity=<\s*(-?\d*),\s*(-?\d)*>")
            .unwrap();
        let caps = re.captures(s).unwrap();
        let pos_x = caps
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .map_err(|_| FormatError)?;
        let pos_y = caps
            .get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .map_err(|_| FormatError)?;
        let vel_x = caps
            .get(3)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .map_err(|_| FormatError)?;
        let vel_y = caps
            .get(4)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .map_err(|_| FormatError)?;
        Ok(Point {
            position: Pair { x: pos_x, y: pos_y },
            velocity: Pair { x: vel_x, y: vel_y },
        })
    }
}

pub fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let body_strs = input.trim().split("\n");
    let mut bodies = body_strs
        .into_iter()
        .map(|b| b.parse::<Point>().unwrap())
        .collect::<Vec<_>>();

    let mut min_x_delta = std::i32::MAX;
    let mut min_y_delta = std::i32::MAX;
    let mut min_bodies = bodies.clone();
    let mut min_overall_x = std::i32::MAX;
    let mut min_overall_y = std::i32::MAX;
    let mut min_i = 0;
    for i in 1..100000 {
        // Get bounding box
        let mut min_x = std::i32::MAX;
        let mut max_x = std::i32::MIN;

        let mut min_y = std::i32::MAX;
        let mut max_y = std::i32::MIN;
        for b in &mut bodies {
            b.position.x += b.velocity.x;
            b.position.y += b.velocity.y;
            if b.position.x < min_x {
                min_x = b.position.x;
            }
            if b.position.x > max_x {
                max_x = b.position.x;
            }
            if b.position.y < min_y {
                min_y = b.position.y;
            }
            if b.position.y > max_y {
                max_y = b.position.y;
            }
        }
        if (max_x - min_x) < min_x_delta && (max_y - min_y) < min_y_delta {
            min_x_delta = max_x - min_x;
            min_y_delta = max_y - min_y;
            min_bodies = bodies.clone();
            min_overall_x = min_x;
            min_overall_y = min_y;
            min_i = i;
        }
    }
    let mut field = vec![vec![" "; (min_x_delta + 1) as usize]; (min_y_delta + 1) as usize];
    for b in &min_bodies {
        let y = b.position.y - min_overall_y;
        let x = b.position.x - min_overall_x;
        field[y as usize][x as usize] = "#";
    }
    for row in field {
        println!("{}", row.join(""));
    }
    println!("time: {}", min_i);
}
