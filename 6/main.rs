use std::io::{BufRead,BufReader};
use std::collections::HashMap;

fn distribute(amount: i32, start: usize, mut mem: Vec<i32>) -> Vec<i32> {
    let mut amount = amount;
    let mut pos = start;
    while amount > 0 {
        mem[pos] += 1;
        amount -= 1;
        pos = (pos + 1) % mem.len();
    }
    mem
}

fn find_next_block(mem: &[i32]) -> usize {
    mem.iter()
        .enumerate()
        .rev()
        .max_by_key(|&p| p.1)
        .unwrap().0
}

fn solve(mut mem: Vec<i32>) -> (usize, usize) {
    let mut seen = HashMap::new();
    let mut c = 0;
    seen.insert(mem.clone(), c);
    loop {
        c += 1;
        let next = find_next_block(&mem);

        let todo = mem[next];
        mem[next] = 0;
        let start = (next + 1) % mem.len();
        mem = distribute(todo, start, mem);
        if let Some(old) = seen.insert(mem.clone(), c) {
            return (c, c - old)
        }
    }
}

fn main() {
    let lines : Result<Vec<String>, _> = BufReader::new(std::io::stdin()).lines().collect();
    let mem : Vec<i32> = lines.unwrap().first().cloned().unwrap().split("\t").map(|s| s.parse::<i32>().unwrap()).collect();
    let s = solve(mem);
    println!("{}", s.0);
    println!("{}", s.1);
}

#[test]
fn distribute_test() {
    assert_eq!(vec![2, 4, 1, 2], distribute(7, 3, vec![0, 2, 0, 0]));
    assert_eq!(vec![3, 1, 2, 3], distribute(4, 2, vec![2, 0, 1, 2]));
}

#[test]
fn find_next_block_test() {
    assert_eq!(2, find_next_block(&vec![0,2,7,0]));
    assert_eq!(1, find_next_block(&vec![2,4,1,2]));
}

#[test]
fn solve_test() {
    assert_eq!((5, 4), solve(vec![0, 2, 7, 0]));
}
