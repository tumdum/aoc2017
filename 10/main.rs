use std::io::{Cursor,Read};
use std::fmt::Write;

fn process(mut data: Vec<u8>, pos: usize, skip: u8) -> Vec<u8> {
    let copy : Vec<u8> = data.iter().chain(data.iter()).skip(pos).take(skip as usize).map(|e| *e).collect();
    for (i, v) in copy.into_iter().rev().enumerate() {
        let real_pos = (pos + i) % data.len();
        data[real_pos] = v;
    }
    data
}

fn solve_a(mut data: Vec<u8>, lengths: &[u8], current_pos: &mut usize, skip_size: &mut usize) -> Vec<u8> {
    for l in lengths.into_iter() {
        data = process(data, *current_pos, *l);
        *current_pos = (*current_pos + (*l as usize) + *skip_size) % data.len();
        *skip_size += 1;
    }
    data
}

fn calc_partial_checksum(input: &[u8]) -> u8 {
    debug_assert!(16 == input.len());
    input.iter().fold(0, |a, b| a ^ b)
}

fn calc_checksum(input: &[u8]) -> Vec<u8> {
    debug_assert!(256 == input.len());
    input.chunks(16).map(calc_partial_checksum).collect()
}

fn checksum_as_hex(input: &[u8]) -> String {
    let mut buf = String::new();
    for b in input.iter() {
        write!(&mut buf, "{:02x}", b).unwrap();
    }
    buf
}

fn solve_b(lengths: &[u8]) -> String {
    let mut data : Vec<_> = (0..256).map(|v| v as u8).collect();
    let mut current_pos = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        data = solve_a(data, lengths, &mut current_pos, &mut skip_size);
    }
    let checksum = calc_checksum(&data);
    debug_assert!(checksum.len() == 16);
    checksum_as_hex(&checksum)
}

fn parse_input_b<R: Read>(r: R) -> Vec<u8> {
    r.bytes().map(|b| b.unwrap() as u8).chain(vec![17,31,73,47,23].into_iter()).map(|b| b as u8).collect()
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let buf = buf.trim();

    let lengths : Vec<u8> = buf.split(",").map(|s| s.parse().unwrap()).collect();
    let data : Vec<_> = (0..256).map(|v| v as u8).collect();
    let solution_a = solve_a(data.clone(), &lengths, &mut 0, &mut 0);
    println!("{}", solution_a[0] as usize * solution_a[1] as usize);

    let solution_b = solve_b(&parse_input_b(Cursor::new(&buf)));
    println!("{}", solution_b);
}

#[test]
fn proces_test() {
    assert_eq!(vec![2, 1, 0, 3, 4], process(vec![0, 1, 2, 3, 4], 0, 3));
    assert_eq!(vec![4, 3, 0, 1, 2], process(vec![2, 1, 0, 3, 4], 3, 4));
    assert_eq!(vec![4, 3, 0, 1, 2], process(vec![4, 3, 0, 1, 2], 3, 1));
    assert_eq!(vec![3, 4, 2, 1, 0], process(vec![4, 3, 0, 1, 2], 1, 5));
}

#[test]
fn solve_a_test() {
    let mut current_pos = 0;
    let mut skip_size = 0;
    assert_eq!(vec![3, 4, 2, 1, 0], solve_a(vec![0, 1, 2, 3, 4], &vec![3, 4, 1, 5], &mut current_pos, &mut skip_size));
    assert_eq!(4, current_pos);
    assert_eq!(4, skip_size);
}

#[test]
fn parse_input_b_test() {
    assert_eq!(vec![49,44,50,44,51,17,31,73,47,23], parse_input_b(Cursor::new("1,2,3")));
}

#[test]
fn calc_partial_checksum_test() {
    let input = vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
    assert_eq!(64, calc_partial_checksum(&input));
}

#[test]
fn checksum_as_hex_test() {
    assert_eq!("4007ff", checksum_as_hex(&[64, 7, 255]));
}

#[test]
fn solve_b_test() {
    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", solve_b(&parse_input_b(Cursor::new(""))));
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", solve_b(&parse_input_b(Cursor::new("AoC 2017"))));
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", solve_b(&parse_input_b(Cursor::new("1,2,3"))));
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", solve_b(&parse_input_b(Cursor::new("1,2,4"))));
}
