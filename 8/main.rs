use std::io::{BufRead,BufReader,stdin};
use std::collections::HashMap;

fn to_rel_fn(input: &str) -> Box<Fn(i32, i32) -> bool> {
    match input {
        ">"     => Box::new(|a, b| a > b),
        ">="    => Box::new(|a, b| a >= b),
        "<"     => Box::new(|a, b| a < b),
        "<="    => Box::new(|a, b| a <= b),
        "=="    => Box::new(|a, b| a == b),
        "!="    => Box::new(|a, b| a != b),
        &_      => panic!("unknown rel: {}", input),
    }
}

struct Cond {
    reg: String,
    rel: Box<Fn(i32, i32) -> bool>,
    val: i32,
}

impl Cond {
    fn eval(&self, regs: &mut Registers) -> bool {
        (self.rel)(regs.get(&self.reg), self.val)
    }
}

enum Op {
    Inc(i32),
    Dec(i32),
}

struct Instr {
    reg: String,
    op: Op,
    cond: Cond,
}

impl Instr {
    fn exec(&self, registers: &mut Registers) {
        if self.cond.eval(registers) {
            let next = registers.get(&self.reg) + match self.op {
                Op::Inc(v) => v,
                Op::Dec(v) => -v,
            };
            registers.set(&self.reg, next);
        }
    }
}

fn parse(line: &str) -> Instr {
    let mut split = line.split_whitespace();
    let reg = split.next().unwrap().to_owned();
    let op_type = split.next().unwrap();
    let op_val = split.next().unwrap().parse().unwrap();
    let op = if op_type == "inc" { Op::Inc(op_val) } else { Op::Dec(op_val) };
    split.next(); // if 
    let cond_reg = split.next().unwrap().to_owned();
    let rel = to_rel_fn(split.next().unwrap());
    let val : i32 = split.next().unwrap().parse().unwrap();
    let cond = Cond{reg: cond_reg, rel: rel, val: val};
    Instr{ reg, op, cond }
}

struct Registers {
    regs: HashMap<String, i32>,
    top: i32,
}

impl Registers {
    fn get(&mut self, reg: &String) -> i32 {
        *self.regs.entry(reg.clone()).or_insert(0)
    }

    fn set(&mut self, reg: &String, val: i32) {
        self.regs.insert(reg.clone(), val);
        self.top = self.top.max(val)
    }

    fn max(&self) -> i32 {
        *self.regs.iter().max_by_key(|&(_, v)| v).unwrap().1
    }
}

fn main() {
    let mut registers = Registers{regs: HashMap::new(), top: i32::min_value()};
    BufReader::new(stdin())
        .lines()
        .map(|l| parse(&l.unwrap()))
        .for_each(|instr| instr.exec(&mut registers));
    println!("{}", registers.max());
    println!("{}", registers.top);
}
