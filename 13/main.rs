use std::io::{BufRead,BufReader,Read};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone)]
struct Scanner {
    range: i32,
    pos: i32,
    dir: i32,
}

impl Debug for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for i in 0..self.range {
            if i == self.pos {
                write!(f, "[S] ")?;
            } else {
                write!(f, "[ ] ")?;
            }
        }
        Ok(())
    }
}

impl Scanner {
    fn new(range: i32) -> Self {
        Scanner{range: range, pos: 0, dir: 1}
    }

    fn tick(&mut self, packet_pos: i32) -> Option<i32> {
        let range = if self.pos == packet_pos {
            Some(self.range)
        } else {
            None
        };
        self.pos += self.dir;
        if self.pos == 0 {
            self.dir = 1;
        } else if self.pos == self.range-1 {
            self.dir = -1;
        }
        range
    }
}

#[derive(Clone)]
struct Firewall {
    scanners: Vec<Option<Scanner>>,
}

impl Debug for Firewall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (i, s) in self.scanners.iter().enumerate() {
            if let &Some(ref s) = s {
                write!(f, "{}: {:?}\n", i, s)?;
            } else {
                write!(f, "{}: ...\n", i)?;
            }
        }
        Ok(())
    }
}

impl Firewall {
    fn new(m: &HashMap<i32, Scanner>) -> Self {
        let max = (m.keys().max().unwrap()+1) as usize;
        let mut scanners : Vec<Option<Scanner>> = vec![None; max];
        for i in 0..max {
            scanners[i] = m.get(&(i as i32)).cloned();
        }
        Firewall{scanners}
    }

    fn tick(&mut self) {
        for (_,scanner) in &mut self.scanners.iter_mut().enumerate() {
            scanner.as_mut().map_or(None, |s| s.tick(0));
        }
    }

    fn deley(&mut self, deley: i32) {
        for _ in 0..deley {
            self.tick();
        }
    }

    fn run(&mut self, deley: i32) -> Option<i32> {
        let mut severity = None;
        self.deley(deley);

        for pos in 0..self.scanners.len() {
            for (i,scanner) in &mut self.scanners.iter_mut().enumerate() {
                let range = scanner.as_mut().map_or(None, |s| s.tick(0));
                if let Some(v) = range {
                    if i == pos {
                        let score = v * pos as i32;
                        severity = Some(severity.unwrap_or(0) + score)
                    }
                }
            }
        }
        severity
    }
}

fn parse_line(s: &str) -> (i32, Scanner) {
    let mut split = s.split(": ");
    let id = split.next().unwrap().parse().unwrap();
    let depth = split.next().unwrap().parse().unwrap();
    (id, Scanner::new(depth))
}

fn parse<R: Read>(r: R) -> HashMap<i32, Scanner> {
    let mut m = HashMap::new();
    for (i, s) in BufReader::new(r).lines().map(|l| parse_line(&l.unwrap())) {
        m.insert(i, s);
    }
    m
}

fn find_deley(m: &HashMap<i32, Scanner>) -> i32 {
    let mut f = Firewall::new(&m);
    for i in 0.. {
        f.tick();
        if f.clone().run(0).is_none() {
            return i+1;
        }
    }
    unreachable!();
}

fn main() {
    let m = parse(std::io::stdin());
    let mut f = Firewall::new(&m);
    println!("{:?}", f.run(0));
    println!("{}", find_deley(&m));
}
