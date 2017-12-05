use std::io::{BufRead,BufReader};

fn execute(instr: &[i32]) -> i32 {
    let mut c = 0;
    let mut buf : Vec<i32> = instr.to_vec();
    let mut pos : i32 = 0;
    while pos < buf.len() as i32 && pos >= 0 {
        c += 1;
        let old = pos as usize;
        pos = pos + buf[pos as usize];
        buf[old] += 1;
    }
    c
}

fn execute_b(instr: &[i32]) -> i32 {
    let mut c = 0;
    let mut buf : Vec<i32> = instr.to_vec();
    let mut pos : i32 = 0;
    while pos < buf.len() as i32 && pos >= 0 {
        c += 1;
        let old = pos as usize;
        pos = pos + buf[pos as usize];
        if buf[old] >= 3 {
            buf[old] -= 1;
        } else {
            buf[old] += 1;
        }
    }
    c
}

fn main() {
    let lines : Result<Vec<String>, _> = BufReader::new(std::io::stdin()).lines().collect();
    let instructions : Vec<i32> = lines.unwrap().into_iter().map(|l| l.parse::<i32>().unwrap()).collect();
    println!("{}", execute(&instructions));
    println!("{}", execute_b(&instructions));
}

#[test]
fn execute_test() {
    assert_eq!(5, execute(&[0, 3, 0, 1, -3]));
}

#[test]
fn execute_b_test() {
    assert_eq!(10, execute_b(&[0, 3, 0, 1, -3]));
}
