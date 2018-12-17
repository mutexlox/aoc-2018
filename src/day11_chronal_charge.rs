fn power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial;
    power *= rack_id;
    power = (power / 100) % 10;
    power -= 5;
    power
}

fn get_region_power(x: i32, y: i32, serial: i32) -> i32 {
    let mut tot = 0;
    for i in x..(x + 3) {
        for j in y..(y + 3) {
            tot += power_level(i, j, serial);
        }
    }
    tot
}

fn main () {
    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let serial = 1308;
    for x in 1..299 {
        for y in 1..299 {
            let power = get_region_power(x, y, serial);
            if power > max_power {
                max_power = power;
                max_x = x;
                max_y = y;
            }
        }
    }
    println!("{},{}", max_x, max_y);
}
