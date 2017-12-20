#![feature(conservative_impl_trait)]

use std::io::{BufRead,BufReader};
use std::collections::HashMap;
use std::ops::Add;

#[derive(Debug,Hash,PartialEq,Eq,Clone,Copy)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn distance(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Ord for Vec3 {
    fn cmp(&self, other: &Vec3) -> std::cmp::Ordering {
        self.distance().cmp(&other.distance())
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Vec3) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_vec(s: &str) -> Vec3 {
    let mut s = s.split(",");
    Vec3{
        x: s.next().unwrap().parse().unwrap(),
        y: s.next().unwrap().parse().unwrap(),
        z: s.next().unwrap().parse().unwrap(),
    }
}

#[derive(Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
struct Particle {
    a: Vec3,
    v: Vec3,
    p: Vec3,
}

impl Particle {
    fn tick(self) -> Particle {
        let v = self.v + self.a;
        let p = self.p + v;
        Particle{a: self.a, v: v, p: p}
    }
}

fn parse_particle(s: &str) -> Particle {
    let mut s = s.split(", ");

    let tmp = s.next().unwrap();
    let p = parse_vec(&tmp[3..tmp.len()-1]);

    let tmp = s.next().unwrap();
    let v = parse_vec(&tmp[3..tmp.len()-1]);

    let tmp = s.next().unwrap();
    let a = parse_vec(&tmp[3..tmp.len()-1]);

    Particle{p, v, a}
}

fn solve_a(particles: &[Particle]) -> Option<usize> {
    particles.iter()
        .enumerate()
        .min_by_key(|&(_,p)| p)
        .map(|t| t.0)
}

fn remove_coliding(particles: Vec<Particle>) -> impl std::iter::Iterator<Item=Particle> {
    let mut positions = HashMap::new();
    for p in &particles {
        *positions.entry(p.p.clone()).or_insert(0) += 1;
    }
    particles.into_iter()
        .filter(move |p| positions.get(&p.p) == Some(&1))
}

fn solve_b(mut particles: Vec<Particle>) -> usize {
    for _ in 0..10000 {
        particles = remove_coliding(particles).map(Particle::tick).collect();
    }
    particles.len()
}

fn main() {
    let particles : Vec<_> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| parse_particle(&l.unwrap()))
        .collect();
    println!("{}", solve_a(&particles).unwrap());
    println!("{}", solve_b(particles));
}
