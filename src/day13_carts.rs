use std::io::{self, Read};

#[derive(Debug, Copy, Clone)]
enum Track {
    Vertical,
    Horizontal,
    BLTRCurve,  // a curve like '/'
    TLBRCurve,  // a curve like '\'
    Intersection,
}

#[derive(Debug, Copy, Clone)]
enum TurnDir {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum FaceDir {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug, Copy, Clone)]
struct Cart {
    next_direction: TurnDir,
    face_direction: FaceDir,
}

impl Cart {
    fn new(face_direction: FaceDir) -> Cart {
        Cart {
            next_direction: TurnDir::Left,
            face_direction
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Slot {
    track: Option<Track>,
    cart: Option<Cart>
}

impl Slot {
    fn new(track: Option<Track>, cart: Option<Cart>) -> Slot {
        Slot {
            track,
            cart,
        }
    }
}

fn new_turn_dir(t: TurnDir) -> TurnDir {
    match t {
        TurnDir::Left => TurnDir::Straight,
        TurnDir::Straight => TurnDir::Right,
        TurnDir::Right => TurnDir::Left,
    }
}

// Computes new face direction for a given turn direction and current face.
fn new_face_from_turn(t: TurnDir, f: FaceDir) -> FaceDir {
    match (t, f) {
        (TurnDir::Left, FaceDir::Up) => FaceDir::Left,
        (TurnDir::Left, FaceDir::Left) => FaceDir::Down,
        (TurnDir::Left, FaceDir::Down) => FaceDir::Right,
        (TurnDir::Left, FaceDir::Right) => FaceDir::Up,
        (TurnDir::Straight, _) => f,
        (TurnDir::Right, FaceDir::Up) => FaceDir::Right,
        (TurnDir::Right, FaceDir::Left) => FaceDir::Up,
        (TurnDir::Right, FaceDir::Down) => FaceDir::Left,
        (TurnDir::Right, FaceDir::Right) => FaceDir::Down,
    }
}

// Evaluate one step, returning the location of a crash, if any.
fn step(map: &mut Vec<Vec<Slot>>) -> Option<(usize, usize)> {
    let mut new_map = map.clone();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if let Some(mut c) = map[i][j].cart {
                let (new_i, new_j) = match c.face_direction {
                    FaceDir::Up => (i - 1, j),
                    FaceDir::Left => (i, j - 1),
                    FaceDir::Right => (i, j + 1),
                    FaceDir::Down => (i + 1, j),
                };
                if map[new_i][new_j].cart.is_some() || new_map[new_i][new_j].cart.is_some() {
                    return Some((new_i, new_j));
                }
                new_map[i][j].cart = None;
                c.face_direction = match (map[new_i][new_j].track, c.face_direction) {
                    (Some(Track::BLTRCurve), FaceDir::Up) => FaceDir::Right,
                    (Some(Track::BLTRCurve), FaceDir::Left) => FaceDir::Down,
                    (Some(Track::BLTRCurve), FaceDir::Down) => FaceDir::Left,
                    (Some(Track::BLTRCurve), FaceDir::Right) => FaceDir::Up,
                    (Some(Track::TLBRCurve), FaceDir::Up) => FaceDir::Left,
                    (Some(Track::TLBRCurve), FaceDir::Left) => FaceDir::Up,
                    (Some(Track::TLBRCurve), FaceDir::Down) => FaceDir::Right,
                    (Some(Track::TLBRCurve), FaceDir::Right) => FaceDir::Down,
                    (Some(Track::Intersection), _) => {
                        let dir = c.next_direction;
                        c.next_direction = new_turn_dir(dir);
                        new_face_from_turn(dir, c.face_direction)
                    }
                    (Some(Track::Vertical), _) => c.face_direction,
                    (Some(Track::Horizontal), _) => c.face_direction,
                    (None, _) => panic!("Cart off track: {} {}", i, j),
                };
                new_map[new_i][new_j].cart = Some(c);
            }
        }
    }
    *map = new_map;
    None
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.trim_matches('\n').split("\n");
    let mut map = Vec::new();
    for (i, line) in lines.enumerate() {
        map.push(Vec::new());
        for c in line.chars() {
            let slot = match c {
                '|' => Slot::new(Some(Track::Vertical), None),
                '-' => Slot::new(Some(Track::Horizontal), None),
                '/' => Slot::new(Some(Track::BLTRCurve), None),
                '\\' => Slot::new(Some(Track::TLBRCurve), None),
                '+' => Slot::new(Some(Track::Intersection), None),
                '^' => Slot::new(Some(Track::Vertical), Some(Cart::new(FaceDir::Up))),
                'v' => Slot::new(Some(Track::Vertical), Some(Cart::new(FaceDir::Down))),
                '<' => Slot::new(Some(Track::Horizontal), Some(Cart::new(FaceDir::Left))),
                '>' => Slot::new(Some(Track::Horizontal), Some(Cart::new(FaceDir::Right))),
                ' ' => Slot::new(None, None),
                c => panic!("Invalid track char '{}'", c),
            };
            map[i].push(slot);
        }
    }
    loop {
        if let Some((x, y)) = step(&mut map) {
            println!("{},{}", y, x);
            break;
        }
    }
}
