use std::io::{BufRead, BufReader};
use std::collections::{HashMap,HashSet};

#[derive(Debug,Clone)]
struct Prog {
    name: String,
    weight: i32, 
    total_weight: i32,
    children: Vec<String>,
}

impl Prog {
    fn is_childless(&self) -> bool {
        self.children.is_empty()
    }
}

fn parse(input: &str) -> Prog {
    let split : Vec<&str> = input.split_whitespace().collect();
    let name = split[0].to_owned();
    let weight = split[1][1..split[1].len()-1].parse::<i32>().unwrap();
    let children = if split.len() == 2 { 
        vec![] 
    } else { 
        split[3..].iter().map(|s| (*s).to_owned().replace(",","")).collect()
    };
    let total_weight = weight;
    Prog{name, weight, total_weight, children}
}

fn solve_a(progs: &[Prog]) -> &str {
    let mut parent = HashMap::new();
    for p in progs {
        for h in &p.children {
            parent.insert(h, p.name.clone());
        }
    }
    &progs.iter().find(|p| !parent.contains_key(&p.name)).unwrap().name
}

fn get_all_parents_of_childless(progs: &HashMap<String, Prog>) -> HashSet<String> {
    progs.iter()
        .filter(|&(_, prog)| !prog.is_childless())
        .filter(|&(_, prog)| prog.children.iter().all(|c| progs.get(c).unwrap().is_childless()))
        .map(|(name, _)| name.to_owned())
        .collect()
}

fn has_unbalanced_children(node: &String, progs: &HashMap<String, Prog>) -> Option<Vec<(i32,i32)>> {
    let mut weights = HashSet::new();
    let mut all_weights = vec![];
    for child in &progs.get(node).unwrap().children {
        let child_prog = progs.get(child).unwrap();
        weights.insert(child_prog.total_weight);
        all_weights.push((child_prog.weight, child_prog.total_weight));
    }
    if weights.len() == 1 { None } else { Some(all_weights) }
}

fn find_correct_weight(weights: &[(i32, i32)]) -> i32 {
    let mut m = HashMap::new();
    for &(_, total) in weights {
        *m.entry(total).or_insert(0) += 1;
    }
    debug_assert!(m.len() == 2);
    let max = m.iter().max_by_key(|&(_, c)| c).unwrap();
    let min = m.iter().min_by_key(|&(_, c)| c).unwrap();
    let diff = max.0 - min.0;
    for &(local, total) in weights {
        if *min.0 == total {
            return local + diff;
        }
    }
    unreachable!();
}

fn solve_b(progs : Vec<Prog>) -> i32 {
    let mut progs_map = HashMap::new();
    for p in &progs {
        progs_map.insert(p.name.clone(), p.clone());
    }

    loop {
        let parents_to_check = get_all_parents_of_childless(&progs_map);
        for parent in &parents_to_check {
            if let Some(weights) = has_unbalanced_children(&parent, &progs_map) {
                return find_correct_weight(&weights);
            }
        }
        for parent in parents_to_check {
            let mut p = progs_map.remove(&parent).unwrap().clone();
            p.total_weight += p.children
                .into_iter()
                .map(|c| progs_map.remove(&c).unwrap().total_weight)
                .sum();
            p.children = vec![];
            progs_map.insert(parent, p);
        }
    }
}

fn main() {
    let progs : Vec<Prog> = BufReader::new(std::io::stdin()).lines().map(|l| parse(&l.unwrap())).collect();
    println!("{}", solve_a(&progs));
    println!("{}", solve_b(progs));
}
