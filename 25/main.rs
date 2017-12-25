#[derive(Debug)]
enum State { A, B, C, D, E, F }

#[derive(Debug)]
struct Machine {
    tape: Vec<bool>,
    state: State,
    position: usize,
    checksum_steps: usize,
}

impl Machine {
    fn new() -> Self {
        let steps = 12683008;
        Machine{
            tape: vec![false; (steps+1)*2],
            state: State::A,
            position: steps+1,
            checksum_steps: steps,
        }
    }

    fn checksum(&self) -> usize {
        self.tape.iter().filter(|b| **b).count()
    }

    fn current_value(&mut self) -> bool {
        self.tape[self.position]
    }

    fn write(&mut self, val: bool) {
        self.tape[self.position] = val;
    }

    fn move_left(&mut self) {
        self.position -= 1;
    }

    fn move_right(&mut self) {
        self.position += 1;
    }

    fn run_till_checksum(&mut self) -> usize {
        for _ in 0..self.checksum_steps {
            match self.state {
                State::A => {
                    if !self.current_value()  {
                        self.write(true);
                        self.move_right();
                        self.state = State::B;
                    } else {
                        self.write(false);
                        self.move_left();
                        self.state = State::B;
                    }
                },
                State::B => {
                    if !self.current_value() {
                        self.write(true);
                        self.move_left();
                        self.state = State::C;
                    } else {
                        self.write(false);
                        self.move_right();
                        self.state = State::E;
                    }
                },
                State::C => {
                    if !self.current_value() {
                        self.write(true);
                        self.move_right();
                        self.state = State::E;
                    } else {
                        self.write(false);
                        self.move_left();
                        self.state = State::D;
                    }
                },
                State::D => {
                    if !self.current_value() {
                        self.write(true);
                        self.move_left();
                        self.state = State::A;
                    } else {
                        self.write(true); // TODO: not needed
                        self.move_left();
                        self.state = State::A;
                    }
                },
                State::E => {
                    if !self.current_value() {
                        self.write(false); // TODO: not needed
                        self.move_right();
                        self.state = State::A;
                    } else {
                        self.write(false);
                        self.move_right();
                        self.state = State::F;
                    }
                },
                State::F => {
                    if !self.current_value() {
                        self.write(true);
                        self.move_right();
                        self.state = State::E;
                    } else {
                        self.write(true); // TODO: not needed;
                        self.move_right();
                        self.state = State::A;
                    }
                },
            }
        }
        self.checksum()
    }
}

fn main() {
    let mut machine = Machine::new();
    let checksum = machine.run_till_checksum();
    println!("{}", checksum);
}
