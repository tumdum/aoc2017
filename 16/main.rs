use std::io::Read;

fn spin(input: String, s: usize) -> String {
    let offset = 16 - s;
    let mut buf : [u8; 16] = [0; 16];
    let mut bytes = input.into_bytes();
    for i in 0..16 {
        buf[i] = bytes[(offset+i)%16];
    }
    bytes.copy_from_slice(&buf);
    unsafe { String::from_utf8_unchecked(bytes) }
}

fn swap(input: String, a: usize, b: usize) -> String {
    let mut s = input.into_bytes();
    s.swap(a, b);
    unsafe { String::from_utf8_unchecked(s) }
}

fn swap_named(input: String, a: char, b: char) -> String {
    let ai = input.find(a).unwrap();
    let bi = input.find(b).unwrap();
    swap(input, ai, bi)
}

#[derive(Debug,PartialEq,Clone,Copy)]
enum Move {
    Spin(usize),
    Swap(usize, usize),
    SwapNamed(char, char),
}

impl<'a> From<&'a str> for Move {
   fn from(s: &'a str) -> Move {
       if s.starts_with("s") {
           return Move::Spin(s[1..].parse().unwrap());
       } else if s.starts_with("x") {
           let mut split = s[1..].split('/');
           let a = split.next().unwrap().parse().unwrap();
           let b = split.next().unwrap().parse().unwrap();
           return Move::Swap(a, b);
       } else if s.starts_with("p") {
           let mut chars = s[1..].chars();
           let a = chars.next().unwrap();
           chars.next();
           let b = chars.next().unwrap();
           return Move::SwapNamed(a, b);
       }
       unreachable!();
   }
}

fn make_move(s: String, m: Move) -> String {
    match m {
        Move::Spin(i) => spin(s, i),
        Move::Swap(a, b) => swap(s, a, b),
        Move::SwapNamed(a, b) => swap_named(s, a, b),
    }
}

fn move_times(start: String, moves: &[Move], times: usize) -> String {
    (0..times).fold(start, |last, _| solve_a(last, moves))
}

fn solve_a(start: String, moves: &[Move]) -> String {
    moves.iter().fold(start, |last, m| make_move(last, *m))
}

fn solve_b(start: String, after_one_move: String, moves: &[Move]) -> String {
    let moves_to_make = 1000000000 % find_cycle_length(&start, after_one_move, moves);
    move_times(start, moves, moves_to_make)
}

fn find_cycle_length(expected: &str, start: String, moves: &[Move]) -> usize {
    let mut c = 1;
    let mut current = start;
    loop {
        current = solve_a(current, moves);
        c += 1;
        if current == expected {
            break;
        }
    }
    c
}

fn main() {
    let mut buf = String::with_capacity(50000);
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let moves : Vec<Move> = buf.trim().split(",").map(|s| s.into()).collect();
    let start = "abcdefghijklmnop";
    let a = solve_a(start.to_owned(), &moves);
    println!("{}", a);
    println!("{}", solve_b(start.to_owned(), a, &moves));
}

#[test]
fn spin_test() {
    assert_eq!("pabcdefghijklmno", spin("abcdefghijklmnop".into(), 1));
}

#[test]
fn swap_test() {
    assert_eq!("eabdc", swap("eabcd".into(), 3, 4));
}

#[test]
fn swap_named_test() {
    assert_eq!("baedc", swap_named("eabdc".into(), 'e', 'b'));
}

#[test]
fn parse_move_test() {
    assert_eq!(Move::Spin(3), "s3".into());
    assert_eq!(Move::Swap(3, 4), "x3/4".into());
    assert_eq!(Move::SwapNamed('e', 'b'), "pe/b".into());
}
