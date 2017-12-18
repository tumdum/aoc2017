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
}

impl<'a> State<'a> {
    fn new(instructions: &'a [Instr], pc: i64) -> Self {
        State{regs: Regs::new(), pc: pc, id: pc, last_played: 0, last_recovered: 0, instructions: instructions}
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

    fn run_one(&mut self, own: &mut VecDeque<i64>, other: &mut VecDeque<i64>) -> End {
        let instr = &self.instructions[self.pc as usize];

        let mut should_move = true;
        match instr {
            &Instr::Snd(ref v) => {
                other.push_back(self.read(v));
            },
            &Instr::Set(r, ref v) => {
                let v = self.read(v);
                self.write(r, v);
            },
            &Instr::Add(r, ref v) => {
                let v = self.read(&Val::Reg(r)) + self.read(v);
                self.write(r, v);
            },
            &Instr::Mul(r, ref v) => {
                let y = self.read(v);
                let x = self.read(&Val::Reg(r));
                let v = x * y;
                self.write(r, v);
            },
            &Instr::Mod(r, ref v) => {
                let y = self.read(v);
                let x = self.read(&Val::Reg(r));
                let v = x % y;
                self.write(r, v);
            },
            &Instr::Rcv(r) => {
                if own.is_empty() {
                    should_move = false;
                    return End::Wait;
                } else {
                    let v = own.pop_front().unwrap();
                    self.write(r, v);
                }
            }
            &Instr::Jgz(ref x, ref y) => {
                if self.read(x) > 0 {
                    self.pc += self.read(y);
                    should_move = false;
                }
            },
        }
        if should_move {
            self.pc += 1;
        }
        if self.pc < 0 || self.pc >= self.instructions.len() as i64 {
            return End::Natural;
        }
        return End::Continue;
    }
}

fn parse(s: &str) -> Instr {
    s.into()
}

#[derive(PartialEq)]
enum End { Continue, Wait, Natural, }

fn run(p: &mut State, mut own: &mut VecDeque<i64>, mut other: &mut VecDeque<i64>) -> End {
    loop {
        let end = p.run_one(&mut own, &mut other);
        if end != End::Continue {
            return end;
        }
    }
}


fn main() {
    let instructions : Vec<Instr> = 
        BufReader::new(std::io::stdin()).lines().map(|s| parse(&s.unwrap())).collect();
    let mut p0 = State::new(&instructions, 0);
    let mut p0_queue = VecDeque::new();
    let mut p1 = State::new(&instructions, 1);
    let mut p1_queue = VecDeque::new();

    let mut p0_pc = p0.pc;
    let mut p1_pc = p1.pc;
    loop {
        let end0 = run(&mut p0, &mut p0_queue, &mut p1_queue);
        let end1 = run(&mut p1, &mut p1_queue, &mut p0_queue);
    }
}

#[test]
fn test_run() {
    use Instr::*;
    use Val::*;
    let input = vec![Set('a',Lit(1)), Add('a',Lit(2)), 
        Mul('a', Reg('a')), Mod('a', Lit(5)), Snd(Reg('a')),
        Set('a', Lit(0)), Rcv(Reg('a')), Jgz(Reg('a'), Lit(-1)),
        Set('a', Lit(1)), Jgz(Reg('a'), Lit(-2))];

    let state = run(&input);
    println!("{:?}", state);
    assert_eq!(state.last_recovered, 4);
}
