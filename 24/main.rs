use std::io::{BufRead,BufReader};
use std::collections::HashSet;

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
struct Component { x: usize, y: usize }

fn parse_component(s: &str) -> Component {
    let mut split = s.split('/');
    Component{x: split.next().unwrap().parse().unwrap(),
        y: split.next().unwrap().parse().unwrap()}
}

impl std::fmt::Debug for Component {
    fn fmt(&self, mut f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(&mut f, "{}/{}", self.x, self.y)?;
        Ok(())
    }
}

fn swap(mut l: Component) -> Component {
    std::mem::swap(&mut l.x, &mut l.y);
    l
}

fn matching(l: &Component, r: &Component) -> Option<Component> {
    if l.y == r.x {
        Some(r.clone())
    } else if l.y == r.y {
        Some(swap(r.clone()))
    } else {
        None
    }
}

#[derive(Debug)]
struct Bridge{score: usize, len: usize}

impl Bridge {
    fn new(components: Vec<Component>) -> Self {
        let score = components.iter().map(|c| c.x + c.y).sum();
        let len = components.len();
        Bridge{score: score, len: len}
    }

    fn len(&self) -> usize { self.len }

    fn score(&self) -> usize { self.score }
}

impl std::hash::Hash for Bridge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.score().hash(state);
        self.len().hash(state);
    }
}

impl Eq for Bridge { }

impl PartialEq for Bridge {
    fn eq(&self, other: &Bridge) -> bool {
        self.score() == other.score() && self.len() == other.len()
    }
}

fn generate_suffixes(prefix: &[Component], components: &HashSet<Component>, mut bridges: &mut HashSet<Bridge>) {
    bridges.insert(Bridge::new(prefix.to_vec()));
    if components.is_empty() {
        return;
    }
    let last = prefix.last().unwrap();
    for component in components {
        if let Some(c) = matching(last, component) {
            let mut next_prefix = prefix.to_vec();
            next_prefix.push(c);
            let mut rest = components.clone();
            rest.remove(component);
            generate_suffixes(&next_prefix, &rest, &mut bridges);
        }
    }
}

fn generate_all(components: HashSet<Component>) -> HashSet<Bridge> {
    let mut bridges = HashSet::new();
    for start in components.iter().filter(|c| c.x == 0 || c.y == 0) {
        let real_start = if start.x == 0 { start.clone() } else { swap(start.clone()) };
        let mut rest = components.clone();
        rest.remove(start);
        generate_suffixes(&[real_start], &rest, &mut bridges);
    }
    bridges
}

fn bridge_cmp(l: &Bridge, r: &Bridge) -> std::cmp::Ordering {
    if l.len() < r.len() {
        return std::cmp::Ordering::Less;
    } else if l.len() > r.len() {
        return std::cmp::Ordering::Greater;
    } else {
        return l.score().cmp(&r.score());
    }
}

fn solve_a(all_bridges: &HashSet<Bridge>) {
    let best = all_bridges.iter().max_by_key(|b| b.score()).unwrap();
    println!("score {} for {:?}", best.score(), best);
}

fn solve_b(all_bridges: &HashSet<Bridge>) {
    let best = all_bridges.iter().max_by(|l,r| bridge_cmp(&l,&r)).unwrap();
    println!("score {} for {:?}", best.score(), best);
}

fn main() {
    let components : HashSet<_> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| parse_component(&l.unwrap()))
        .collect();
    let all_bridges = generate_all(components);
    println!("distinct bridges: {}", all_bridges.len());
    solve_a(&all_bridges);
    solve_b(&all_bridges);
}
