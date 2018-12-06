extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

fn max_from_arr(arr: &[u8]) -> (usize, &u8) {
    arr.iter().enumerate().max_by_key(|(_, t)| *t).unwrap()
}

fn main() {
    let datetime_re = Regex::new(r"^\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\].*$").unwrap();
    let guard_re = Regex::new(r"^.*Guard #(\d+) begins shift$").unwrap();
    let sleep_str = "falls asleep";
    let wake_str = "wakes up";

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let entries = input.trim().split("\n");

    let mut sleeps = HashMap::<i32, [u8; 60]>::new();
    let mut sleep_times = HashMap::<i32, i32>::new();
    let mut id = -1;
    let mut sleep_time = 0;
    for entry in entries {
        // Input was pre-processed (sorted chronologically)
        let caps = datetime_re.captures(entry).unwrap();

        let minute: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        if let Some(c) = guard_re.captures(&entry) {
            id = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
        } else if entry.contains(sleep_str) {
            sleep_time = minute;
        } else {
            assert!(entry.contains(wake_str));
            let arr = sleeps.entry(id).or_insert([0; 60]);
            let time = sleep_times.entry(id).or_insert(0);
            *time += minute as i32 - sleep_time as i32;
            for i in sleep_time..minute {
                arr[i] += 1;
            }
        }
    }
    let sleepy_id = *sleep_times.iter().max_by_key(|(_, v)| *v).unwrap().0;
    let sleepy_time = max_from_arr(sleeps.get(&sleepy_id).unwrap()).0;
    println!(
        "Guard {} slept most of any guard and slept most at {}; product = {}",
        sleepy_id,
        sleepy_time,
        sleepy_id * (sleepy_time as i32)
    );
}
