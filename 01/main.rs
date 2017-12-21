use std::io::Read;

fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn sum(input: &[u32]) -> u32 {
    let ret = input
        .windows(2)
        .map(|w| if w[0] == w[1] { w[0] } else { 0 })
        .sum();
    if input.last() == input.first() { 
        ret + input.last().unwrap() 
    } else { 
        ret 
    }
}

fn pos(size: usize, pos: usize) -> usize {
    (pos + size/2) % size
}

fn sumB(input: &[u32]) -> u32 {
    let size = input.len();
    input
        .iter()
        .enumerate()
        .map(|(i, v)| { let a = input[pos(size, i)]; if a == *v { a } else { 0 }})
        .sum()
}

fn main() {
    let mut input = std::io::stdin();
    let mut buf = String::new();
    input.read_to_string(&mut buf);
    println!("{}", sum(&parse(buf.trim())));
    println!("{}", sumB(&parse(buf.trim())));
}

#[test]
fn parse_test() {
    assert_eq!(parse("1122"), vec![1,1,2,2]);
    assert_eq!(parse("1111"), vec![1,1,1,1]);
    assert_eq!(parse("1234"), vec![1,2,3,4]);
    assert_eq!(parse("91212129"), vec![9,1,2,1,2,1,2,9]);
}

#[test]
fn parse_sum() {
    assert_eq!(3, sum(&[1,1,2,2]));
    assert_eq!(4, sum(&[1,1,1,1]));
    assert_eq!(0, sum(&[1,2,3,4]));
    assert_eq!(9, sum(&[9,1,2,1,2,1,2,9]));
}

#[test]
fn parse_sumB() {
    assert_eq!(6, sumB(&[1,2,1,2]));
    assert_eq!(0, sumB(&[1,2,2,1]));
    assert_eq!(4, sumB(&[1,2,3,4,2,5]));
    assert_eq!(12, sumB(&[1,2,3,1,2,3]));
    assert_eq!(4, sumB(&[1,2,1,3,1,4,1,5]));
}
