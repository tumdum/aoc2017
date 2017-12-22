use std::io::{BufRead,BufReader,Read};
use std::collections::{HashMap,HashSet};

type Pos = (i64, i64);

enum Rotation { Left, Right }

#[derive(Clone, Debug)]
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

    fn reverse(&self) -> Orientation {
        match self {
            &Orientation::Up => Orientation::Down,
            &Orientation::Down => Orientation::Up,
            &Orientation::Left => Orientation::Right,
            &Orientation::Right => Orientation::Left,
        }
    }
}

#[derive(Clone, Debug)]
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

    fn reverse(&mut self) {
        self.orientation = self.orientation.reverse();
    }
}

#[derive(Clone,PartialEq)]
enum State { Clean, Weakened, Infected, Flagged }

impl State {
    fn next(&self) -> State {
        match self {
            &State::Clean => State::Weakened,
            &State::Weakened => State::Infected,
            &State::Infected => State::Flagged,
            &State::Flagged => State::Clean,
        }
    }
}

#[derive(Clone)]
struct Grid {
    states: HashMap<Pos, State>,
    infections: usize,
}

impl Grid {
    fn new(s: HashSet<Pos>) -> Grid {
        let mut m = HashMap::new();
        for p in s {
            m.insert(p, State::Infected);
        }
        Grid{states: m, infections: 0}
    }

    fn get_state(&self, pos: &Pos) -> State {
        self.states.get(pos).cloned().unwrap_or(State::Clean)
    }

    fn touch(&mut self, pos: &Pos) {
        let current_state = self.get_state(pos);
        let next_state = current_state.next();
        if next_state == State::Infected {
            self.infections += 1;
        }
        self.states.insert(pos.clone(), next_state);
    }


    fn burst_a(&mut self, mut c: Carrier) -> Carrier {
        let current_infected = self.get_state(&c.pos) != State::Clean;
        if current_infected {
            c.rotate(Rotation::Right);
            self.states.remove(&c.pos);
        } else {
            c.rotate(Rotation::Left);
            self.states.insert(c.pos.clone(), State::Infected);
            self.infections += 1;
        }
        c.move_forward();
        c
    }
    fn burst_b(&mut self, mut c: Carrier) -> Carrier {
        let current_state = self.get_state(&c.pos);
        match current_state {
            State::Clean => c.rotate(Rotation::Left),
            State::Weakened => {},
            State::Infected => c.rotate(Rotation::Right),
            State::Flagged => c.reverse(),
        }

        self.touch(&c.pos);
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

fn solve_a(mut g: Grid, mut c: Carrier, steps: usize) -> usize {
    for _ in 0..steps {
        c = g.burst_a(c);
    }
    g.infections
}

fn solve_b(mut g: Grid, mut c: Carrier, steps: usize) -> usize {
    for _ in 0..steps {
        c = g.burst_b(c);
    }
    g.infections
}

fn main() {
    let (infected, start) = parse(std::io::stdin());
    let grid = Grid::new(infected);
    let carrier = Carrier{pos: start, orientation: Orientation::Up};
    solve_a(grid.clone(), carrier.clone(), 10000);
    println!("a: {:10} => {}", 10000, solve_a(grid.clone(), carrier.clone(), 10000));
    solve_b(grid.clone(), carrier.clone(), 10000000);
    println!("b: {:10} => {}", 10000000, solve_b(grid, carrier, 10000000));
}
