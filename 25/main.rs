use std::collections::HashMap;
use std::cell::*;

enum Direction { Left, Right }

#[derive(Hash,PartialEq,Eq)]
enum State { A, B, C, D, E, F }

struct Description {
    off_action: Action,
    on_action: Action,
}

fn i2b(i: usize) -> bool { i == 1 }

impl Description {
    fn new(v_off: usize, d_off: Direction, s_off: State, v_on: usize, d_on: Direction, s_on: State) -> Self {
        Description{
            off_action: Action::new(i2b(v_off), d_off, s_off),
            on_action: Action::new(i2b(v_on), d_on, s_on),
        }
    }
}

struct Action {
    value_to_set: bool,
    direction_to_take: Direction,
    state_to_transition: State,
}

impl Action {
    fn new(value_to_set: bool, direction_to_take: Direction, state_to_transition: State) -> Self {
        Action{value_to_set, direction_to_take, state_to_transition}
    }
}

struct Machine {
    tape: Vec<bool>,
    state: State,
    position: usize,
    checksum_steps: usize,
    descriptions: HashMap<State, RefCell<Description>>,
}

impl Machine {
    fn new() -> Self {
        use Direction::*;
        use State::*;

        let a_desc = RefCell::new(Description::new(1, Right, B,  0, Left, B));
        let b_desc = RefCell::new(Description::new(1, Left, C,   0, Right, E));
        let c_desc = RefCell::new(Description::new(1, Right, E,  0, Left, D));
        let d_desc = RefCell::new(Description::new(1, Left, A,   1, Left, A));
        let e_desc = RefCell::new(Description::new(0, Right, A,  0, Right, F));
        let f_desc = RefCell::new(Description::new(1, Right, E,  1, Right, A));

        let mut descriptions = HashMap::new();
        descriptions.insert(A, a_desc);
        descriptions.insert(B, b_desc);
        descriptions.insert(C, c_desc);
        descriptions.insert(D, d_desc);
        descriptions.insert(E, e_desc);
        descriptions.insert(F, f_desc);

        let steps = 12683008;
        Machine{
            tape: vec![false; (steps+1)*2],
            state: State::A,
            position: steps+1,
            checksum_steps: steps,
            descriptions: descriptions,
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

    fn move_to(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.position -= 1,
            Direction::Right => self.position += 1,
        }
    }

    fn run(&mut self) -> usize {
        for _ in 0..self.checksum_steps {
            let value = self.current_value();
            let action : Ref<_> = if value {
                self.descriptions[&self.state].borrow()
            } else {
                self.descriptions[&self.state].borrow()
            };
            if value {
                self.execute(action);
            } else {
                self.execute(action);
            }
        }
        self.checksum()
    }

    fn execute(&self, d: Ref<Description>) {
    }

    fn run_till_checksum(&mut self) -> usize {
        for _ in 0..self.checksum_steps {
            match self.state {
                State::A => {
                    if !self.current_value()  {
                        self.write(true);
                        self.move_to(Direction::Right);
                        self.state = State::B;
                    } else {
                        self.write(false);
                        self.move_to(Direction::Left);
                        self.state = State::B;
                    }
                },
                State::B => {
                    if !self.current_value() {
                        self.write(true);
                        self.move_to(Direction::Left);
                        self.state = State::C;
                    } else {
                        self.write(false);
                        self.move_to(Direction::Right);
                        self.state = State::E;
                    }
                },
                State::C => {
                    if !self.current_value() {
                        self.write(true);
                        self.move_to(Direction::Right);
                        self.state = State::E;
                    } else {
                        self.write(false);
                        self.move_to(Direction::Left);
                        self.state = State::D;
                    }
                },
                State::D => {
                    if !self.current_value() {
                        self.write(true);
                        self.move_to(Direction::Left);
                        self.state = State::A;
                    } else {
                        self.write(true); // TODO: not needed
                        self.move_to(Direction::Left);
                        self.state = State::A;
                    }
                },
                State::E => {
                    if !self.current_value() {
                        self.write(false); // TODO: not needed
                        self.move_to(Direction::Right);
                        self.state = State::A;
                    } else {
                        self.write(false);
                        self.move_to(Direction::Right);
                        self.state = State::F;
                    }
                },
                State::F => {
                    if !self.current_value() {
                        self.write(true);
                        self.move_to(Direction::Right);
                        self.state = State::E;
                    } else {
                        self.write(true); // TODO: not needed;
                        self.move_to(Direction::Right);
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
    let checksum = machine.run();
    println!("{}", checksum);
}
