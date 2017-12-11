// great description at https://www.redblobgames.com/grids/hexagons/

use std::io::Read;

#[derive(Clone)]
enum Dir { N, NE, SE, S, SW, NW }

impl<'a> From<&'a str> for Dir {
    fn from(s: &'a str) -> Dir {
        match s {
            "n"     => Dir::N, "ne"    => Dir::NE, "se"    => Dir::SE,
            "s"     => Dir::S, "sw"    => Dir::SW, "nw"    => Dir::NW,
            &_      => panic!("unknown dir '{}'", s),
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
struct AxialPos { q: i32, r: i32, }

impl AxialPos {
    fn move_by(&self, d: Dir) -> AxialPos {
        match d {
            Dir::N      => AxialPos{q: self.q,   r: self.r-1},
            Dir::NE     => AxialPos{q: self.q+1, r: self.r-1},
            Dir::SE     => AxialPos{q: self.q+1, r: self.r},
            Dir::S      => AxialPos{q: self.q,   r: self.r+1},
            Dir::SW     => AxialPos{q: self.q-1, r: self.r+1},
            Dir::NW     => AxialPos{q: self.q-1, r: self.r},
        }
    }
}

impl AxialPos {
    fn distance(&self, other: &AxialPos) -> i32 {
        let qself : QubePos = self.into();
        let qother : QubePos = other.into();
        qself.distance(qother)
    }
}

struct QubePos { x: i32, y: i32, z: i32, }

impl QubePos {
    fn distance(&self, b: QubePos) -> i32 {
        let a = self;
        *[(a.x-b.x).abs(), (a.y-b.y).abs(), (a.z-b.z).abs()].into_iter().max().unwrap()
    }
}

impl<'a> From<&'a AxialPos> for QubePos {
    fn from(a: &'a AxialPos) -> Self {
        QubePos{x: a.q, y: -a.q-a.r, z: a.r}
    }
}

fn move_by(start: &AxialPos, directions: &[Dir]) -> (AxialPos,AxialPos) {
    let mut max = start.clone();
    let mut max_distance = 0;
    let mut end = start.clone();
    for d in directions {
        end = end.move_by(d.clone());
        let distance = start.distance(&end);
        if distance > max_distance {
            max_distance = distance;
            max = end.clone();
        }
    }
    (end, max)
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let buf = buf.trim();
    let dirs : Vec<Dir> = buf.split(",").map(|s| s.into()).collect();
    let start = AxialPos{q: 0, r: 0};
    let solution = move_by(&start, &dirs);
    println!("{}", start.distance(&solution.0));
    println!("{}", start.distance(&solution.1));
}

#[test]
fn pos_move_test() {
    let f = |q, r| AxialPos{q, r};
    let start = f(0, 0);
    use Dir::*;
    assert_eq!(3, move_by(&start, &[NE,NE,NE]).0.distance(&start));
    assert_eq!(0, move_by(&start, &[NE,NE,SW,SW]).0.distance(&start));
    assert_eq!(2, move_by(&start, &[NE,NE,S,S]).0.distance(&start));
    assert_eq!(3, move_by(&start, &[SE,SW,SE,SW,SW]).0.distance(&start));
}
