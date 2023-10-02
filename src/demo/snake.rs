#![allow(dead_code)]

use rand::Rng;

#[derive(Debug)]
struct Coord {
    x: u8,
    y: u8,
}

impl Coord {
    fn new() -> Coord {
        let mut rng = rand::thread_rng();
        let x: u8 = rng.gen_range(0..10);
        let y: u8 = rng.gen_range(0..10);
        Coord { x, y }
    }
}

#[derive(Debug)]
struct Snake {
    head: Coord,
    body: Vec<Coord>,
}

impl Snake {
    pub fn new() -> Self {
        Self { head: Coord::new(), body: Vec::new() }
    }
    pub fn walk(&mut self, new_coord: Coord) {
        self.head = new_coord;
    }
}

#[test]
fn case_rng() {
    let mut a = Snake::new();
    println!("{:#?}", a);

    let b = Coord::new();
    a.walk(b);
    println!("{:#?}", a);
}
