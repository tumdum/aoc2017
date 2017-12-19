use std::io::{BufRead,BufReader,Read};

type Seen = Vec<Pos>;

type Grid = Vec<Vec<char>>;

fn parse<R: Read>(r: R) -> Grid {
    let r = BufReader::new(r);
    let g : Vec<Vec<_>> = 
        r.lines().map(|l| l.unwrap().chars().collect()).collect();
    g
}

type Pos = (i32, i32);

fn find_start(g: &Grid) -> Pos {
    for (i, c) in g[0].iter().enumerate() {
        if *c == '|' {
            return (i as i32, 0);
        }
    }
    unreachable!();
}

fn get_letter(p: Pos, g: &Grid) -> Option<char> {
    let x = p.0;
    let y = p.1;
    if x < 0 || y < 0 {
        return None;
    }
    let x = x as usize;
    let y = y as usize;
    g.get(y).and_then(|r| r.get(x)).cloned()
}

fn record_path(visited: &mut Vec<Pos>, next: Pos) {
    match visited.last().cloned() {
        None => visited.push(next),
        Some(other) => {
            if other != next {
                visited.push(next);
            }
        }
    }
}

type F = Fn(Pos) -> Pos;

fn find_crossroad(start: Pos, direction: Box<F>, g: &Grid, mut visited: &mut Seen) -> Option<(Pos, Pos, Vec<char>)> {
    record_path(&mut visited, start);

    let mut letters = vec![];
    let mut current = start;
    let mut last = current;
    get_letter(current, &g).into_iter().filter(|c| c.is_alphabetic()).for_each(|c| letters.push(c));

    while let Some(v) = get_letter(direction(current), &g) {
        last = current;
        current = direction(current);
        if v == ' ' {
            return None;
        }
        record_path(&mut visited, current);
        if v == '+' {
            break;
        } else if v.is_alphabetic() {
            letters.push(v);
        }
    }
    Some((current, last, letters))
}

fn select_next(p: Pos, last: Pos, g: &Grid, mut visited: &mut Seen) -> Option<(Pos, Box<F>)> {
    let fs : Vec<Box<F>> =
        vec![Box::new(up), Box::new(down), Box::new(left), Box::new(right)];
    for f in fs.into_iter() {
        let next = f(p);
        match get_letter(next, &g) {
            None => continue,
            Some(v) => {
                if next == last || v == ' ' {
                    continue;
                }
                record_path(&mut visited, next);
                return Some((next, f));
            }
        }
    }
    None
}

fn up(p: Pos) -> Pos { (p.0, p.1-1) }
fn down(p: Pos) -> Pos { (p.0,p.1+1) }
fn left(p: Pos) -> Pos { (p.0-1, p.1) }
fn right(p: Pos) -> Pos { (p.0+1, p.1) }

fn solve_a(g: &Grid) -> (String, usize) {
    let mut visited = Seen::new();
    let mut current = find_start(&g);
    record_path(&mut visited, current);
    let mut letters = vec![];
    let mut f : Box<F> = Box::new(down);
    while let Some(crossroads) = find_crossroad(current, f, &g, &mut visited) {
        letters.extend(crossroads.2);
        match select_next(crossroads.0, crossroads.1, &g, &mut visited) {
            None => break,
            Some(next) => {
                current = next.0;
                f = next.1;
            },
        }
    }
    (letters.into_iter().collect(), visited.len())
}

fn main() {
    let g = parse(std::io::stdin());
    println!("{:?}", solve_a(&g));
}
