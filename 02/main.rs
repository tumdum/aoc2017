use std::io::{BufRead,BufReader,Cursor,Read};

type Spreadsheet = Vec<Vec<i32>>;

fn parse<R: Read>(r: R) -> Spreadsheet {
    let r = BufReader::new(r);
    r.lines()
        .map(|s| s.unwrap()
             .split_whitespace()
             .map(|s| s.parse::<i32>().unwrap())
             .collect() )
        .collect()
}

fn row_checksum_a<T: AsRef<[i32]>>(r: T) -> i32 {
    r.as_ref().iter().max().unwrap() - r.as_ref().iter().min().unwrap()
}

fn row_checksum_b<T: AsRef<[i32]>>(r: T) -> i32 {
    for a in r.as_ref().iter() {
        for b in r.as_ref().iter() {
            if a == b {
                continue
            }
            let (l, m) = if a < b { (a, b) } else { (b, a) };
            let d = m / l;
            if d * l == *m {
                return d
            }
        }
    }
    unreachable!();
}

fn checksum<F: FnMut(&Vec<i32>) -> i32>(f: F, s: Spreadsheet) -> i32 {
    s.iter().map(f).sum()
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    println!("{}", checksum(|r| row_checksum_a(r), parse(Cursor::new(&buf))));
    println!("{}", checksum(|r| row_checksum_b(r), parse(Cursor::new(&buf))));
}

#[test]
fn parse_test() {
    let input = "5 1 9 5
    7 5 3
    2 4 6 8";
    let spreadsheet = parse(std::io::Cursor::new(input));
    assert_eq!(vec![vec![5,1,9,5], vec![7,5,3], vec![2,4,6,8]], spreadsheet);
}

#[test]
fn checksum_test() {
    let input = vec![vec![5,1,9,5], vec![7,5,3], vec![2,4,6,8]];
    assert_eq!(18, checksum(|r| row_checksum_a(r), input));
}

#[test]
fn row_checksum_b_test() {
    assert_eq!(4, row_checksum_b(&vec![5, 9, 2, 8]));
    assert_eq!(3, row_checksum_b(&vec![9, 4, 7, 3]));
    assert_eq!(2, row_checksum_b(&vec![3, 8, 6, 5]));
}
