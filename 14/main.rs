use std::fmt::Write;
use std::collections::{HashSet,VecDeque};

const IN_SUFFIX : [u8; 5] = [17, 31, 73, 47, 23];

fn calc_partial_checksum(input: &[u8]) -> u8 {
    input.iter().fold(0, |a, b| a ^ b)
}

fn to_bit_aux(b: u8) -> Vec<bool> {
    let ret : Vec<bool> = format!("{:04b}", b)
        .bytes()
        .map(|b| (b as char) == '1')
        .collect();
    ret
}

fn to_bit(b: &u8) -> Vec<bool> {
    let nstart = '0' as u8;
    let nlast = '9' as u8;
    if *b >= nstart && *b <= nlast {
        return to_bit_aux(*b - nstart);
    }
    let cstart = 'a' as u8;
    to_bit_aux(10 + *b - cstart)
}

fn to_bits(input: &[u8]) -> Vec<bool> {
    input.iter().flat_map(to_bit).collect()
}

fn to_bits_string(input: &str) -> Vec<bool> {
    to_bits(&input.bytes().collect::<Vec<u8>>())
}

fn checksum_as_hex(input: &[u8]) -> String {
    let mut buf = String::new();
    for b in input.iter() {
        write!(&mut buf, "{:02x}", b).unwrap();
    }
    buf
}

fn process(mut data: Vec<u8>, pos: usize, skip: u8) -> Vec<u8> {
    let copy : Vec<u8> = data.iter().chain(data.iter()).skip(pos).take(skip as usize).map(|e| *e).collect();
    for (i, v) in copy.into_iter().rev().enumerate() {
        let real_pos = (pos + i) % data.len();
        data[real_pos] = v;
    }
    data
}

fn calc_checksum(input: &[u8]) -> Vec<u8> {
    input.chunks(16).map(calc_partial_checksum).collect()
}

fn hash_part(mut data: Vec<u8>, lengths: &[u8], current_pos: &mut usize, skip_size: &mut usize) -> Vec<u8> {
    for l in lengths.into_iter() {
        data = process(data, *current_pos, *l);
        *current_pos = (*current_pos + (*l as usize) + *skip_size) % data.len();
        *skip_size += 1;
    }
    data
}
fn knot_hash(lengths: &[u8]) -> String {
    let lengths : Vec<_> = lengths.iter().chain(IN_SUFFIX.iter()).map(|e| *e).collect();
    let data : Vec<_> = (0..256).map(|v| v as u8).collect();
    let mut current_pos = 0;
    let mut skip_size = 0;
    let data = (0..64).fold(data, |d, _| hash_part(d, &lengths, &mut current_pos, &mut skip_size));
    let checksum = calc_checksum(&data);
    checksum_as_hex(&checksum)
}

type Grid = Vec<Vec<bool>>;

fn build_grid(prefix: &str) -> Grid {
    (0..128)
        .map(|i| format!("{}-{}", prefix, i))
        .map(|s| knot_hash(&s.bytes().collect::<Vec<u8>>()))
        .map(|s| to_bits_string(&s))
        .collect()
}

fn solve_a(grid: &Grid) -> i32 {
    grid
        .iter()
        .map(|row| row.iter().filter(|b| **b).count() as i32)
        .sum()
}

type Pos = (i32, i32);

fn get_s<T>(v: &[T], i: i32) -> Option<&T> {
    if i >= v.len() as i32 || i < 0 {
        None
    } else {
        Some(&v[i as usize])
    }
}

fn get(grid: &Vec<Vec<bool>>, p: &Pos) -> bool {
    *get_s(&grid, p.1).and_then(|r| get_s(r, p.0)).unwrap_or(&false)
}

fn find_next_region_start(seen: &HashSet<Pos>, grid: &Grid) -> Option<(i32, i32)> {
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            let pos = (x as i32, y as i32);
            if !seen.contains(&pos) && get(&grid, &pos) {
                return Some(pos);
            }
        }
    }
    None
}

fn up(p: &Pos) -> Pos { (p.0, p.1-1) }
fn down(p: &Pos) -> Pos { (p.0, p.1+1) }
fn left(p: &Pos) -> Pos { (p.0-1, p.1) }
fn right(p: &Pos) -> Pos { (p.0+1, p.1) }

fn get_region(start: Pos, grid: &Grid) -> HashSet<Pos> {
    let mut region = HashSet::new();
    let mut to_check = VecDeque::new();
    to_check.push_back(start);
    while let Some(p) = to_check.pop_front() {
        region.insert(p);
        let mut f = |p :&Pos| {
            if get(&grid, p) && !region.contains(&p) {
                region.insert(*p);
                to_check.push_back(*p);
            }
        };
        f(&left(&p));
        f(&right(&p));
        f(&up(&p));
        f(&down(&p));
    }
    region
}

fn solve_b(grid: &Grid) -> i32 {
    let mut seen = HashSet::new();
    let mut regions = 0;
    while let Some(start) = find_next_region_start(&seen, &grid) {
        let region = get_region(start, &grid);
        seen.extend(region.into_iter());
        regions += 1;
    }
    regions
}

fn main() {
    let grid = build_grid("hwlqcszp");
    println!("{}", solve_a(&grid));
    println!("{}", solve_b(&grid));
}

#[test]
fn solve_a_test() {
    assert_eq!(8108, solve_a(&build_grid("flqrgnkx")));
}

#[test]
fn solve_b_test() {
    assert_eq!(1242, solve_b(&build_grid("flqrgnkx")));
}
