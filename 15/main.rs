const MASK : u64 = 0b1111_1111_1111_1111;
const FACTOR_A : u64 = 16807;
const FACTOR_B : u64 = 48271;
const INPUT_A : u64 = 289;
const INPUT_B : u64 = 629;

struct Generator { factor: u64, previous: u64, filter: u64, }

impl Generator {
    fn next(&mut self) -> u64 {
        while {
            self.previous = (self.previous * self.factor) % 2147483647;
            self.previous % self.filter != 0
        } {}
        self.previous
    }
}

fn judge(mut a: Generator, mut b: Generator, count: usize) -> usize {
    (0..count).map(|_| (a.next() & MASK) == (b.next() & MASK)).filter(|b| *b).count()
}

fn solve_a() -> usize {
    let gen_a = Generator{factor: FACTOR_A, previous: INPUT_A, filter: 1};
    let gen_b = Generator{factor: FACTOR_B, previous: INPUT_B, filter: 1};
    judge(gen_a, gen_b, 40_000_000)
}

fn solve_b() -> usize {
    let gen_a = Generator{factor: FACTOR_A, previous: INPUT_A, filter: 4};
    let gen_b = Generator{factor: FACTOR_B, previous: INPUT_B, filter: 8};
    judge(gen_a, gen_b, 5_000_000)
}

fn main() {
    println!("{}", solve_a());
    println!("{}", solve_b());
}
