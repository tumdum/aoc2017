use std::io::{BufRead,BufReader,Read};
use std::collections::HashSet;

type Pos = (i64, i64);

enum Rotation { Left, Right }

#[derive(Clone)]
enum Orientation { Up, Down, Left, Right }

impl Orientation {
    fn move_pos(&self, p: Pos) -> Pos {
        match self {
            &Orientation::Up => (p.0, p.1 - 1),
            &Orientation::Down => (p.0, p.1 + 1),
            &Orientation::Left => (p.0 - 1, p.1),
            &Orientation::Right => (p.0 + 1, p.1),
        }
    }

    fn rotate(&self, rot: Rotation) -> Orientation {
        match (self, rot) {
            (&Orientation::Up, Rotation::Left) => Orientation::Left,
            (&Orientation::Left, Rotation::Left) => Orientation::Down,
            (&Orientation::Down, Rotation::Left) => Orientation::Right,
            (&Orientation::Right, Rotation::Left) => Orientation::Up,
            (_, Rotation::Right) => self
                .rotate(Rotation::Left).rotate(Rotation::Left).rotate(Rotation::Left),
        }
    }
}

#[derive(Clone)]
struct Carrier {
    pos: Pos,
    orientation: Orientation,
}

impl Carrier {
    fn rotate(&mut self, rot: Rotation) {
        self.orientation = self.orientation.rotate(rot);
    }
    
    fn move_forward(&mut self) {
        self.pos = self.orientation.move_pos(self.pos);
    }
}

#[derive(Clone)]
struct Grid {
    infected: HashSet<Pos>,
    infections: usize,
}

impl Grid {
    fn burst(&mut self, mut c: Carrier) -> Carrier {
        let current_infected = self.infected.contains(&c.pos);
        if current_infected {
            c.rotate(Rotation::Right);
            self.infected.remove(&c.pos);
        } else {
            c.rotate(Rotation::Left);
            self.infected.insert(c.pos);
            self.infections += 1;
        }
        c.move_forward();
        c
    }
}

fn parse<R: Read>(r: R) -> (HashSet<Pos>, Pos) {
    let r = BufReader::new(r);
    let g : Vec<Vec<bool>> = 
        r.lines().map(|l| l.unwrap().chars().map(|c| c == '#').collect()).collect();
    let mut s = HashSet::new();
    let center = ((g[0].len()/2) as i64, (g.len()/2) as i64);
    for y in 0..g.len() {
        let row_len = g[y].len();
        for x in 0..row_len {
            if g[y][x] {
                s.insert((x as i64, y as i64));
            }
        }
    }
    (s, center)
}

fn solve_a(mut g: Grid, mut c: Carrier, steps: usize) {
    for _ in 0..steps {
        c = g.burst(c);
    }
    println!("{} => {}", steps, g.infections);
}

fn main() {
    let (infected, start) = parse(std::io::stdin());
    println!("{:?} {:?}", infected, start);
    let grid = Grid{infected: infected, infections: 0};
    let carrier = Carrier{pos: start, orientation: Orientation::Up};
    // solve_a(grid.clone(), carrier.clone(), 7);
    // solve_a(grid.clone(), carrier.clone(), 70);
    solve_a(grid.clone(), carrier.clone(), 10000);
}
