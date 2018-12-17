fn power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial;
    power *= rack_id;
    power = (power / 100) % 10;
    power -= 5;
    power
}

fn get_region_power(cache: &mut Vec<Vec<i32>>, x: i32, y: i32, serial: i32, size: i32) -> i32 {
    let mut tot = cache[(x - 1) as usize][(y - 1) as usize];
    for i in x..(x + size) {
        let j = y + size - 1;
        tot += power_level(i, j, serial);
    }
    for j in y..(y + size - 1) {
        let i = x + size - 1;
        tot += power_level(i, j, serial);
    }
    cache[(x - 1) as usize][(y - 1) as usize] = tot;
    tot
}

fn main() {
    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_size = 0;
    let serial = 1308;
    let mut cache = vec![vec![0; 300]; 300];
    for size in 1..301 {
        for x in 1..301 - size + 1 {
            for y in 1..301 - size + 1 {
                let power = get_region_power(&mut cache, x, y, serial, size);
                if power > max_power {
                    max_power = power;
                    max_x = x;
                    max_y = y;
                    max_size = size;
                }
            }
        }
    }
    println!("{},{},{}", max_x, max_y, max_size);
}
