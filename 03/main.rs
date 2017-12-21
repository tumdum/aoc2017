use std::collections::HashMap;
/*

17  16  15  14  13
18   5   4   3  12
19   6   1   2  11 ..
20   7   8   9  10 27
21  22  23  24  25 26

*/

#[derive(Eq,PartialEq,Debug)]
struct CircleSize {
    total_count: i32,
    side_size: i32,
}

fn inner_size(circles: i32) -> CircleSize {
    if circles == 0 {
        return CircleSize{total_count: 1, side_size: 1}
    }
    let prev = inner_size(circles - 1);
    let side_size = prev.side_size + 2;
    let total_count = side_size * side_size;
    CircleSize{total_count, side_size}
}

#[derive(Clone,Copy,PartialEq,Debug)]
struct Position {
    x: i32,
    y: i32,
    side: i32,
}

impl Position {
    fn new(x: i32, y: i32, side: i32) -> Position {
        Position{x, y, side}
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn neighbours(&self) -> Vec<Position> {
        let mut ret = vec![];
        for xo in vec![-1,0,1] {
            for yo in vec![-1, 0, 1] {
                if xo == 0 && yo == 0 {
                    continue
                }
                ret.push(Position{x: self.x + xo, y: self.y + yo, side: self.side})
            }
        }
        debug_assert!(ret.len() == 8);
        ret
    }

    fn next(&self) -> Position {
        let offset = self.side / 2;
        if self.x == offset {
            if self.y == offset {
                return Position{x: self.x-1, y:self.y, side:self.side}
            }
            return Position{x: self.x, y: self.y+1, side: self.side}
        } else if self.x == -offset {
            if self.y == -offset {
                return Position{x: self.x+1, y:self.y, side:self.side}
            }
            return Position{x: self.x, y: self.y-1, side:self.side}
        } else if self.y == -offset {
            return Position{x: self.x + 1, y: self.y, side: self.side}
        }
        return Position{x: self.x - 1, y: self.y, side: self.side}
    }
}

fn number_to_position(input: i32) -> Position {
    if input == 1 {
        return Position{x: 0, y: 0, side: 1}
    }
    let mut remaining = input;
    let mut i = 1;
    loop {
        let prev_loop = inner_size(i-1);
        let next_loop = inner_size(i);
        if remaining < next_loop.total_count {
            remaining -= prev_loop.total_count;
            break;
        }
        i += 1;
    }
    let mut pos = start(i);
    if remaining == 0 {
        let end = Position{x: pos.x-1, y: pos.y, side: pos.side};
        return end;
    }
    for _ in 0..remaining-1 {
        pos = pos.next();
    }
    pos
}

fn solve(input: i32) -> i32 {
    number_to_position(input).distance()
}

struct Grid {
    data: HashMap<(i32, i32), i32>,
}

impl Grid {
    fn new() -> Self {
        Grid{data: HashMap::new()}
    }

    fn get(&self, pos: Position) -> Option<i32> {
        self.data.get(&(pos.x, pos.y)).cloned()
    }

    fn set(&mut self, n: i32) -> i32 {
        let pos = number_to_position(n);
        let neighbours = pos.neighbours();
        let sum = neighbours.into_iter().map(|n| self.get(n).unwrap_or(0)).sum();
        let sum = if sum == 0 { 1 } else { sum };
        self.data.insert((pos.x, pos.y), sum);
        sum
    }
}

fn solve_b(input: i32) -> i32 {
    let mut g = Grid::new();
    for i in 1.. {
        let v = g.set(i);
        if v > input {
            return v
        }
    }
    unreachable!();
}

fn main() {
    println!("{}", solve(361527));
    println!("{}", solve_b(361527));
}

#[test]
fn inner_size_test() {
    assert_eq!(CircleSize{total_count:1, side_size: 1},
               inner_size(0));
    assert_eq!(CircleSize{total_count:9, side_size: 3},
               inner_size(1));
    assert_eq!(CircleSize{total_count:25, side_size: 5}, 
               inner_size(2));
    assert_eq!(CircleSize{total_count:49, side_size: 7}, 
               inner_size(3));
}

fn start(circle: i32) -> Position {
    Position{x: circle, y: 1 - circle, side: 2 * circle + 1}
}

#[test]
fn start_test() {
    assert_eq!(Position::new(1,  0, 3), start(1));
    assert_eq!(Position::new(2, -1, 5), start(2));
    assert_eq!(Position::new(3, -2, 7), start(3));
}


#[test]
fn position_next_test() {
    let p5 = |x, y| Position{x: x, y: y, side: 5};
    assert_eq!(p5(1, 2), p5(2, 2).next());
    assert_eq!(p5(2,-1), p5(2,-2).next());
    assert_eq!(p5(-1,-2), p5(-2,-2).next());
    assert_eq!(p5(-2,1), p5(-2,2).next());

    assert_eq!(p5(2,1), p5(2,0).next());
    assert_eq!(p5(-2,-1), p5(-2,0).next());

    assert_eq!(p5(1,-2), p5(0,-2).next());
    assert_eq!(p5(-1,2), p5(0,2).next());
}

#[test]
fn solve_test() {
    assert_eq!(0, solve(1));

    assert_eq!(1, solve(2));
    assert_eq!(2, solve(3));
    assert_eq!(1, solve(4));
    assert_eq!(2, solve(5));
    assert_eq!(1, solve(6));
    assert_eq!(2, solve(7));
    assert_eq!(1, solve(8));
    assert_eq!(2, solve(9));

    assert_eq!(3, solve(10));
    assert_eq!(2, solve(11));
    assert_eq!(3, solve(12));

    assert_eq!(2, solve(23));

    assert_eq!(31, solve(1024));

    assert_eq!(326, solve(361527));
}

#[test]
fn test_grid_set() {
    let mut g = Grid::new();
    assert_eq!(1, g.set(1));
    assert_eq!(1, g.set(2));
    assert_eq!(2, g.set(3));
    assert_eq!(4, g.set(4));
    assert_eq!(5, g.set(5));
    assert_eq!(10, g.set(6));
    assert_eq!(11, g.set(7));
    assert_eq!(23, g.set(8));
    assert_eq!(25, g.set(9));
    assert_eq!(26, g.set(10));
}
