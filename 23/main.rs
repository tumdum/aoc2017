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
    Set(char, Val),
    Sub(char, Val),
    Mul(char, Val),
    Jnz(Val, Val),
}

impl<'a> From<&'a str> for Instr {
    fn from(s: &'a str) -> Instr {
        let mut split = s.split_whitespace();
        let first = split.next().unwrap();
        if first.starts_with("set") {
            let second = split.next().unwrap();
            let third = split.next().unwrap();
            return Instr::Set(second.chars().next().unwrap(), third.into());
        } else if first.starts_with("sub") {
            let second = split.next().unwrap();
            let third = split.next().unwrap();
            return Instr::Sub(second.chars().next().unwrap(), third.into());
        } else if first.starts_with("mul") {
            let second = split.next().unwrap();
            let third = split.next().unwrap();
            return Instr::Mul(second.chars().next().unwrap(), third.into());
        } else if first.starts_with("jnz") {
            let second = split.next().unwrap();
            let third = split.next().unwrap();
            return Instr::Jnz(second.into(), third.into());
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
    muls: i32
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
            muls: 0};
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

    fn sub(&mut self, r: char, v: &Val) {
        let v = self.read(&Val::Reg(r)) - self.read(v);
        self.write(r, v);
    }

    fn mul(&mut self, r: char, v: &Val) {
        let y = self.read(v);
        let x = self.read(&Val::Reg(r));
        let v = x * y;
        self.write(r, v);
        self.muls += 1;
    }

    fn jnz(&mut self, x: &Val, y: &Val) -> bool {
        if self.read(x) != 0 {
            self.pc += self.read(y);
            return true;
        }
        return false;
    }

    fn run(&mut self) -> bool {
        let instr = &self.instructions[self.pc as usize];

        let mut jumped = false;
        match instr {
            &Instr::Set(r, ref v) => self.set(r, v),
            &Instr::Sub(r, ref v) => self.sub(r, v),
            &Instr::Mul(r, ref v) => self.mul(r, v),
            &Instr::Jnz(ref x, ref y) => jumped = self.jnz(x, y),
        }
        if !jumped {
            self.pc += 1;
        }
        if self.pc < 0 || self.pc >= self.instructions.len() as i64 {
            return false;
        }
        return true;
    }
}

fn parse(s: &str) -> Instr {
    s.into()
}

fn solve_a(instructions: &[Instr]) {
    let mut state = State::new(&instructions, 0);
    while state.run() { }
    println!("{}", state.muls);
}

fn main() {
    let instructions : Vec<_> = 
        BufReader::new(std::io::stdin()).lines().map(|s| parse(&s.unwrap())).collect();
    solve_a(&instructions);
}
