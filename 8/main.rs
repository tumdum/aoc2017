use std::io::{BufRead,BufReader,stdin};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Rel {
    Gt,
    GtEq,
    Lt,
    LtEq,
    Eq,
    Ne,
}

impl<'a> From<&'a str> for Rel {
    fn from(input: &str) -> Rel {
        use Rel::*;
        match input {
            ">"     => Gt,
            ">="    => GtEq,
            "<"     => Lt,
            "<="    => LtEq,
            "=="    => Eq,
            "!="    => Ne,
            &_      => panic!("unknown rel: {}", input),
        }
    }
}

impl Rel {
    fn eval(&self, l: i32, r: i32) -> bool {
        use Rel::*;
        match self {
            &Gt => l > r,
            &GtEq => l >= r,
            &Lt => l < r,
            &LtEq => l <= r,
            &Eq => l == r,
            &Ne => l != r,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Cond {
    reg: String,
    rel: Rel,
    val: i32,
}

impl Cond {
    fn eval(&self, regs: &mut Registers) -> bool {
        let reg_val = regs.get(&self.reg);
        self.rel.eval(reg_val, self.val)
    }
}

#[derive(PartialEq, Debug)]
enum Op {
    Inc(i32),
    Dec(i32),
}

#[derive(PartialEq, Debug)]
struct Instr {
    reg: String,
    op: Op,
    cond: Cond,
}

impl Instr {
    fn exec(&self, registers: &mut Registers) {
        if self.cond.eval(registers) {
            let current = registers.get(&self.reg);
            let next = match self.op {
                Op::Inc(v) => current + v,
                Op::Dec(v) => current - v,
            };
            registers.set(&self.reg, next);
        }
    }
}

fn parse(line: &str) -> Instr {
    let mut split = line.split_whitespace();
    let reg = split.next().unwrap().to_owned();
    let op1 = split.next().unwrap();
    let op2 = split.next().unwrap().parse().unwrap();
    let op = if op1 == "inc" { Op::Inc(op2) } else { Op::Dec(op2) };
    split.next(); // if 
    let cond_reg = split.next().unwrap().to_owned();
    let rel : Rel = split.next().unwrap().into();
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
        if self.top < val {
            self.top = val
        }
    }

    fn max(&self) -> i32 {
        *self.regs.iter().max_by_key(|&(_, v)| v).unwrap().1
    }
}

fn main() {
    let lines : Result<Vec<String>, _> = BufReader::new(stdin()).lines().collect();
    let instructions = lines.unwrap().into_iter().map(|l| parse(&l));
    let mut registers = Registers{regs: HashMap::new(), top: i32::min_value()};
    for instr in instructions {
        instr.exec(&mut registers);
    }
    println!("{}", registers.max());
    println!("{}", registers.top);
}

#[test]
fn parse_test() {
    assert_eq!(Instr{reg: "b".to_owned(), op: Op::Inc(5), cond: Cond{reg: "a".to_owned(), rel: Rel::Gt, val: 1}},
        parse("b inc 5 if a > 1"));
    assert_eq!(Instr{reg: "a".to_owned(), op: Op::Inc(1), cond: Cond{reg: "b".to_owned(), rel: Rel::Lt, val: 5}},
        parse("a inc 1 if b < 5"));
    assert_eq!(Instr{reg: "c".to_owned(), op: Op::Dec(-10), cond: Cond{reg: "a".to_owned(), rel: Rel::GtEq, val: 1}},
        parse("c dec -10 if a >= 1"));
    assert_eq!(Instr{reg: "c".to_owned(), op: Op::Inc(-20), cond: Cond{reg: "c".to_owned(), rel: Rel::Eq, val: 10}},
        parse("c inc -20 if c == 10"));
}
