use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone)]
struct FormatError;

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid format for input; expect x, y")
    }
}

impl error::Error for FormatError {
    fn description(&self) -> &str {
        "invalid format for input; expect x, y"
    }
    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl FromStr for Point {
    type Err = FormatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let no_whitespace: String = s.replace(" ", "");
        let pieces = no_whitespace.split(",");
        let ints_parsed = pieces
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| FormatError)?;

        if ints_parsed.len() != 2 {
            Err(FormatError)
        } else {
            Ok(Point {
                x: ints_parsed[0],
                y: ints_parsed[1],
            })
        }
    }
}

fn get_closest_to(source: Point, points: &Vec<Point>) -> Option<usize> {
    let mut closest = 0;
    let mut valid = true;
    for i in 0..points.len() {
        if points[i].dist(&source) < points[closest].dist(&source) {
            closest = i;
            valid = true;
        } else if i != closest && points[i].dist(&source) == points[closest].dist(&source) {
            valid = false;
        }
    }
    if valid {
        Some(closest)
    } else {
        None
    }
}

fn num_within_max_dist(
    points: &Vec<Point>,
    max_dist: i32,
    left: &Point,
    right: &Point,
    top: &Point,
    bot: &Point,
) -> i32 {
    let mut count = 0;
    for x in left.x..right.x + 1 {
        for y in top.y..bot.y + 1 {
            let mut tot = 0;
            for point in points {
                tot += point.dist(&Point{x: x, y: y});
            }
            if tot < max_dist {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let inputs = input.split("\n").map(|s| s.trim()).filter(|s| s.len() != 0);
    let points: Vec<_> = inputs.map(|inp| inp.parse::<Point>().unwrap()).collect();

    let mut left = &points[0];
    let mut right = &points[0];
    let mut top = &points[0];
    let mut bot = &points[0];
    for p in &points {
        if p.x < left.x {
            left = p;
        }
        if p.x > right.x {
            right = p;
        }
        if p.y < top.y {
            top = p;
        }
        if p.y > bot.y {
            bot = p;
        }
    }

    let mut empty_to_point_map = HashMap::<(i32, i32), usize>::new();
    let mut area_map = HashMap::<usize, Option<usize>>::new();

    for x in left.x..right.x + 1 {
        for y in top.y..bot.y + 1 {
            if let Some(closest_idx) = get_closest_to(Point { x: x, y: y }, &points) {
                empty_to_point_map.insert((x, y), closest_idx);
                if x == left.x || x == right.x || y == top.y || y == bot.y {
                    // Infinite area.
                    area_map.insert(closest_idx, None);
                }
            }
        }
    }

    for idx in empty_to_point_map.values() {
        if let Some(v) = area_map.entry(*idx).or_insert(Some(0)) {
            *v += 1;
        }
    }
    let max_area = area_map.values().max();
    println!("max area: {}", max_area.unwrap().unwrap());

    let max_dist = 10000;
    println!(
        "number within total dist {} to all points: {}",
        max_dist,
        num_within_max_dist(&points, max_dist, left, right, top, bot)
    );
}
