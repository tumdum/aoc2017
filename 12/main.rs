use std::io::{BufRead,BufReader,stdin};
use std::collections::{HashMap,HashSet};

fn parse_line(s: &str) -> (i32, Vec<i32>) {
    let mut seg = s.split_whitespace();
    let id = seg.next().unwrap().parse().unwrap();
    seg.next();
    let ids = seg.map(|c| c.replace(",","").parse().unwrap()).collect();
    (id, ids)
}

fn is_connected(neighbours: &[i32], rest: &HashSet<i32>) -> bool {
    neighbours.iter().any(|n| rest.contains(n))
}

fn solve_a(mut data: HashMap<i32, Vec<i32>>) -> usize {
    find_group(&mut data, 0).len()
}

fn find_group(m: &mut HashMap<i32, Vec<i32>>, root: i32) -> HashSet<i32> {
    let mut connected = HashSet::new();
    connected.insert(root);
    assert!(m.remove(&root).is_some());

    loop {
        if let Some(k) = m.iter().find(|&(_, v)| is_connected(v, &connected)).map(|p| *p.0) {
            connected.insert(k);
            m.remove(&k);
        } else {
            break;
        }
    }

    connected
}

fn solve_b(mut m: HashMap<i32, Vec<i32>>) -> i32 {
    let mut groups = 0;
    loop {
        if let Some(k) = m.iter().next().map(|p| *p.0) {
            find_group(&mut m, k).into_iter().for_each(|k| { m.remove(&k); });
            groups += 1;
        } else {
            break;
        }
    }
    groups
}

fn main() {
    let mut m = HashMap::new();
    BufReader::new(stdin())
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .for_each(|l| { m.insert(l.0, l.1.clone()); });
    println!("{}", solve_a(m.clone()));
    println!("{}", solve_b(m));
}
