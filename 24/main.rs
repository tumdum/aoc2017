use std::io::{BufRead,BufReader};
use std::collections::HashSet;

#[derive(Clone,PartialEq,Eq,Hash)]
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

type Bridge = Vec<Component>;

fn generate_suffixes(prefix: &[Component], components: &HashSet<Component>) -> Vec<Bridge> {
    if components.is_empty() {
        return vec![prefix.to_vec()];
    }
    let last = prefix.last().unwrap();
    let mut bridges = vec![prefix.to_vec()];
    for component in components {
        if let Some(c) = matching(last, component) {
            let mut next_prefix = prefix.to_vec();
            next_prefix.push(c);
            let mut rest = components.clone();
            rest.remove(component);
            let result = generate_suffixes(&next_prefix, &rest);
            bridges.extend(result.into_iter());
        }
    }
    bridges
}

fn generate_all(components: HashSet<Component>) -> Vec<Bridge> {
    let mut bridges = vec![];
    for start in components.iter().filter(|c| c.x == 0 || c.y == 0) {
        let real_start = if start.x == 0 { start.clone() } else { swap(start.clone()) };
        let mut rest = components.clone();
        rest.remove(start);
        bridges.extend(generate_suffixes(&[real_start], &rest).into_iter());
    }
    bridges
}

fn bridge_score(bridge: &Bridge) -> usize {
    bridge.iter().map(|c| c.x + c.y).sum()
}

fn bridge_cmp(l: &Bridge, r: &Bridge) -> std::cmp::Ordering {
    if l.len() < r.len() {
        return std::cmp::Ordering::Less;
    } else if l.len() > r.len() {
        return std::cmp::Ordering::Greater;
    } else {
        return bridge_score(l).cmp(&bridge_score(r));
    }
}

fn solve_a(all_bridges: &[Bridge]) {
    let best = all_bridges.iter().max_by_key(|b| bridge_score(&b)).unwrap();
    println!("score {} for {:?}", bridge_score(&best), best);
}

fn solve_b(all_bridges: &[Bridge]) {
    let best = all_bridges.iter().max_by(|l,r| bridge_cmp(&l,&r)).unwrap();
    println!("score {} for {:?}", bridge_score(&best), best);
}

fn main() {
    let components : HashSet<_> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| parse_component(&l.unwrap()))
        .collect();
    let all_bridges = generate_all(components);
    solve_a(&all_bridges);
    solve_b(&all_bridges);
}
