use std::io::{BufRead,BufReader};
use std::collections::{HashMap,VecDeque};

type Regs = HashMap<char, i64>;

#[derive(Debug)]
enum Val {
    Reg(char),
    Lit(i64),
}

impl<'a> From<&'a str> for Val {
    fn from(s: &'a str) -> Val {
        if let Ok(v) = s.parse() {
            return Val::Lit(v);
        }
        Val::Reg(s.chars().next().unwrap())
    }
}

#[derive(Debug)]
enum Instr {
    Snd(Val),
    Set(char, Val),
    Add(char, Val),
    Mul(char, Val),
    Mod(char, Val),
    Rcv(char),
    Jgz(Val, Val),
}

impl<'a> From<&'a str> for Instr {
    fn from(s: &'a str) -> Instr {
        let mut split = s.split_whitespace();
        let first = split.next().unwrap();
        if first.starts_with("snd") {
            return Instr::Snd(split.next().unwrap().into());
        } else if first.starts_with("set") {
            let second = split.next().unwrap();
            let third = split.next().unwrap();
            return Instr::Set(second.chars().next().unwrap(), third.into());
        } else if first.starts_with("add") {
            let second = split.next().unwrap();
            let third = split.next().unwrap();
            return Instr::Add(second.chars().next().unwrap(), third.into());
        } else if first.starts_with("mul") {
            let second = split.next().unwrap();
            let third = split.next().unwrap();
            return Instr::Mul(second.chars().next().unwrap(), third.into());
        } else if first.starts_with("mod") {
            let second = split.next().unwrap();
            let third = split.next().unwrap();
            return Instr::Mod(second.chars().next().unwrap(), third.into());
        } else if first.starts_with("rcv") {
            return Instr::Rcv(split.next().unwrap().chars().next().unwrap());
        } else if first.starts_with("jgz") {
            let second = split.next().unwrap();
            let third = split.next().unwrap();
            return Instr::Jgz(second.into(), third.into());
        }
        unreachable!();
    }
}

#[derive(Debug)]
struct State<'a> {
    regs: Regs,
    pc: i64,
    id: i64,

    last_played: i64,
    last_recovered: i64,
    instructions: &'a [Instr],
    sends: i32
}

impl<'a> State<'a> {
    fn new(instructions: &'a [Instr], id: i64) -> Self {
        let mut s = State{
            regs: Regs::new(), 
            pc: 0, 
            id: id, 
            last_played: 0,
            last_recovered: 0,
            instructions: instructions, 
            sends: 0};
        s.write('p', id);
        s
    }

    fn read(&mut self, val: &Val) -> i64 {
        match val {
            &Val::Lit(i) => i,
            &Val::Reg(n) => *self.regs.entry(n).or_insert(0),
        }
    }

    fn write(&mut self, reg: char, val: i64) {
        self.regs.insert(reg, val);
    }

    fn set(&mut self, r: char, v: &Val) {
        let v = self.read(v);
        self.write(r, v);
    }

    fn add(&mut self, r: char, v: &Val) {
        let v = self.read(&Val::Reg(r)) + self.read(v);
        self.write(r, v);
    }

    fn mul(&mut self, r: char, v: &Val) {
        let y = self.read(v);
        let x = self.read(&Val::Reg(r));
        let v = x * y;
        self.write(r, v);
    }

    fn modulo(&mut self, r: char, v: &Val) {
        let y = self.read(v);
        let x = self.read(&Val::Reg(r));
        let v = x % y;
        self.write(r, v);
    }

    fn jgz(&mut self, x: &Val, y: &Val) -> bool {
        if self.read(x) > 0 {
            self.pc += self.read(y);
            return true;
        }
        return false;
    }

    fn run_sound(&mut self) -> bool {
        let instr = &self.instructions[self.pc as usize];

        let mut jumped = false;
        match instr {
            &Instr::Set(r, ref v) => self.set(r, v),
            &Instr::Add(r, ref v) => self.add(r, v),
            &Instr::Mul(r, ref v) => self.mul(r, v),
            &Instr::Mod(r, ref v) => self.modulo(r, v),
            &Instr::Snd(ref v) => self.last_played = self.read(v),
            &Instr::Jgz(ref x, ref y) => jumped = self.jgz(x, y),
            &Instr::Rcv(r) => {
                if self.read(&Val::Reg(r)) != 0 {
                    self.last_recovered = self.last_played;
                    return false;
                }
            }
        }
        if !jumped {
            self.pc += 1;
        }
        if self.pc < 0 || self.pc >= self.instructions.len() as i64 {
            return false;
        }
        return true;
    }

    fn run_one(&mut self, own: &mut VecDeque<i64>, other: &mut VecDeque<i64>) -> bool {
        if self.pc < 0 || self.pc >= self.instructions.len() as i64 {
            return false;
        }

        let instr = &self.instructions[self.pc as usize];

        let mut jumped = false;
        match instr {
            &Instr::Set(r, ref v) => self.set(r, v),
            &Instr::Add(r, ref v) => self.add(r, v),
            &Instr::Mul(r, ref v) => self.mul(r, v),
            &Instr::Mod(r, ref v) => self.modulo(r, v),
            &Instr::Jgz(ref x, ref y) => jumped = self.jgz(x, y),
            &Instr::Snd(ref v) => {
                let v = self.read(v);
                other.push_back(v);
                self.sends += 1;
            },
            &Instr::Rcv(r) => {
                if own.is_empty() {
                    return false;
                } else {
                    let v = own.pop_front().unwrap();
                    self.write(r, v);
                }
            }
        }
        if !jumped {
            self.pc += 1;
        }
        return true;
    }
}

fn parse(s: &str) -> Instr {
    s.into()
}

fn run(p: &mut State, mut own: &mut VecDeque<i64>, mut other: &mut VecDeque<i64>) -> i64 {
    let mut executed_instructions = 0;
    while p.run_one(&mut own, &mut other) { executed_instructions += 1; }
    executed_instructions
}

fn solve_a(instructions: &[Instr]) {
    let mut state = State::new(&instructions, 0);
    while state.run_sound() { }
    println!("{}", state.last_recovered);
}

fn solve_b(instructions: &[Instr]) {
    let mut p0 = State::new(&instructions, 0);
    let mut p0_queue = VecDeque::new();
    let mut p1 = State::new(&instructions, 1);
    let mut p1_queue = VecDeque::new();

    loop {
        let ic0 = run(&mut p0, &mut p0_queue, &mut p1_queue);
        let ic1 = run(&mut p1, &mut p1_queue, &mut p0_queue);
        if ic0 == 0 && ic1 == 0 {
            break;
        }
    }
    println!("{}", p1.sends);
}

fn main() {
    let instructions : Vec<_> = 
        BufReader::new(std::io::stdin()).lines().map(|s| parse(&s.unwrap())).collect();
    solve_a(&instructions);
    solve_b(&instructions);
}
