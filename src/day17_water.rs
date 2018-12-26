use std::collections::HashMap;
use std::io::{self, Read};
use std::ops::Range;

fn get_range(s: &str) -> Range<usize> {
    if s.contains("..") {
        let pairs = s.split("..").collect::<Vec<_>>();
        pairs[0].parse::<usize>().unwrap()..pairs[1].parse::<usize>().unwrap() + 1
    } else {
        let x = s.parse::<usize>().unwrap();
        x..x + 1
    }
}

const SPRING_X: usize = 500;
const SPRING_Y: usize = 0;

fn explore(map: &mut HashMap<(usize, usize), char>, x: usize, y: usize, y_max: usize) {
    if y > y_max {
        return;
    }
    // No-ops.
    if let Some('#') = map.get(&(x, y)) {
        return;
    }
    if let Some('~') = map.get(&(x, y)) {
        return;
    }
    map.insert((x, y), '|');

    // Go down as far as possible.
    explore(map, x, y + 1, y_max);

    if y + 1 > y_max {
        return;
    }

    // Since we explored, below is already set.
    let below = *map.get(&(x, y + 1)).unwrap();

    if below == '#' || below == '~' {
        // Spread left and right.
        if !map.contains_key(&(x - 1, y)) {
            explore(map, x - 1, y, y_max);
        }
        if !map.contains_key(&(x + 1, y)) {
            explore(map, x + 1, y, y_max);
        }
        let mut closed_right = false;
        let mut i = x;
        while let Some(c) = map.get(&(i, y)) {
            if *c == '|' {
                i += 1;
            } else if *c == '#' || *c == '~' {
                closed_right = true;
                break;
            } else {
                break;
            }
        }
        let mut closed_left = false;
        i = x;
        while let Some(c) = map.get(&(i, y)) {
            if *c == '|' || *c == '~' {
                i -= 1;
            } else if *c == '#' {
                closed_left = true;
                break;
            } else {
                break;
            }
        }
        if closed_left && closed_right {
            map.insert((x, y), '~');
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let assigs = input.trim().split("\n").map(|l| l.split(","));

    // Build representation of clay locations
    let mut map = HashMap::new();
    let mut y_min = std::usize::MAX;
    let mut y_max = 0;
    for pair in assigs {
        let mut x_range = 0..0;
        let mut y_range = 0..0;
        for a in pair {
            let v = a.trim().split("=").collect::<Vec<_>>();
            let range = get_range(v[1]);
            if v[0] == "x" {
                x_range = range;
            } else {
                y_range = range;
            }
        }
        for x in x_range {
            for y in y_range.clone() {
                if y < y_min {
                    y_min = y;
                } else if y > y_max {
                    y_max = y;
                }
                map.insert((x, y), '#');
            }
        }
    }

    explore(&mut map, SPRING_X, SPRING_Y, y_max);

    println!(
        "{}",
        map.iter()
            .filter(|((_, y), c)| *y >= y_min && *y <= y_max && (**c == '~' || **c == '|'))
            .count()
    );
}
