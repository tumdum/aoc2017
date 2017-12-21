use std::io::{BufRead,BufReader};
use std::collections::{HashSet,HashMap};

type Grid = Vec<Vec<bool>>;

fn parse_grid(s: &str) -> Grid {
    s.split('/')
        .map(|s| s.chars().map(|c| c != '.').collect())
        .collect()
}

fn rotate(g: &Grid) -> Grid {
    let size = g.len();
    let mut out = vec![vec![false; size]; size];
    for y in 0..size {
        for x in 0..size {
            out[y][x] = g[size-x-1][y];
        }
    }
    out
}

fn flip(g: &Grid) -> (Grid, Grid) {
    let l = g.len();
    let mut c1 = g.clone();
    let mut c2 = g.clone();
    for y in 0..l {
        for x in 0..l/2 {
            let tmp = c1[y][x];
            c1[y][x] = c1[y][l-x-1];
            c1[y][l-x-1] = tmp;
        }
    }
    for y in 0..l/2 {
        for x in 0..l {
            let tmp = c2[y][x];
            c2[y][x] = c2[l-y-1][x];
            c2[l-y-1][x] = tmp;
        }
    }
    (c1, c2)
}

#[derive(Debug)]
struct Rule {
    inputs: HashSet<Grid>,
    output: Grid,
}

impl Rule {
    fn new(input: Grid, output: Grid) -> Rule {
        let f1 = flip(&input);

        let r1 = rotate(&input);
        let f2 = flip(&r1);

        let r2 = rotate(&r1);
        let f3 = flip(&r2);

        let r3 = rotate(&r2);
        let f4 = flip(&r3);
        assert!(input == rotate(&r3));
        let tmp = vec![input, r1, r2, r3, f1.0, f1.1, f2.0, f2.1, f3.0, f3.1, f4.0, f4.1];
        let mut inputs = HashSet::new();
        inputs.extend(tmp.into_iter());
        Rule{inputs, output}
    }
}

fn parse_rule(s: &str) -> (Grid, Grid) {
    let mut top = s.split(" => ");
    let left = top.next().unwrap();
    let right = top.next().unwrap();
    (parse_grid(&left), parse_grid(&right))
}

struct RuleSet {
    mapping: HashMap<Grid, Grid>,
}

impl RuleSet {
    fn new(rules: Vec<Rule>) -> RuleSet {
        let mut ret = RuleSet{mapping: HashMap::new()};
        for rule in rules {
            for input in rule.inputs {
                ret.mapping.insert(input, rule.output.clone());
            }
        }
        ret
    }

    fn get(&self, input: &Grid) -> &Grid {
        self.mapping.get(input).unwrap()
    }
}

fn extract(g: &Grid, x_start: usize, y_start: usize, size: usize) -> Grid {
    let mut ret = vec![];
    for y in y_start..(y_start+size) {
        let mut row = vec![];
        for x in x_start..(x_start+size) {
            row.push(g[y][x]);
        }
        ret.push(row);
    }
    ret
}

fn split(g: &Grid) -> Vec<Vec<Grid>> {
    let step = if g.len()%2 == 0 { 2 } else { 3 };
    let mut ret = vec![];
    let mut row_start = 0;
    loop {
        if row_start >= g.len() {
            break;
        }
        let mut col_start = 0;
        let mut row = vec![];
        loop {
            if col_start >= g.len() {
                break;
            }
            row.push(extract(&g, col_start, row_start, step));
            col_start += step;
        }
        ret.push(row);
        row_start += step;
    }
    ret
}

fn merge<'a, I: Iterator<Item=Vec<&'a Grid>>>(input: I) -> Grid {
    let mut output = Vec::with_capacity(input.size_hint().1.unwrap_or(0) * 3);
    for row in input {
        let size = row[0].len();
        for y in 0..size {
            let mut out_row = Vec::with_capacity(row.len() * size);
            for m in &row {
                for x in 0..size {
                    out_row.push(m[y][x])
                }
            }
            output.push(out_row);
        }
    }
    output
}

fn round(g: &Grid, rs: &RuleSet) -> Grid {
    merge(split(g)
            .into_iter()
            .map(|row| row.into_iter().map(|ref e| rs.get(e)).collect()))
}

fn print(g: &Grid) {
    g.iter()
        .flat_map(|r| r.iter().map(|c| if *c { &'#' } else { &'.' }).chain(['\n'].iter()))
        .for_each(|c| print!("{}", c));
}

fn count(g: &Grid) -> usize {
    g.iter().map(|r| r.iter().filter(|b| **b).count()).sum()
}

fn main() {
    let rules : Vec<_> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| parse_rule(&l.unwrap()))
        .map(|p| Rule::new(p.0, p.1))
        .collect();

    let rule_set = RuleSet::new(rules);

    let start = parse_grid(".#./..#/###");
    let mut a = start.clone();
    for _ in 0..5 {
        a = round(&a, &rule_set);
    }
    print(&a);
    println!("{}", count(&a));

    let mut b = a;
    for _ in 0..(18-5) {
        b = round(&b, &rule_set);
    }
    println!("{}", count(&b));
}
