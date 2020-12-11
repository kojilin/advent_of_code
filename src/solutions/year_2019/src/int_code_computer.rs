pub struct IntCodeComputer {
    program: Vec<i32>,
    offset: usize,
    halt: bool,
}

impl IntCodeComputer {
    pub fn new(codes: &Vec<i32>) -> IntCodeComputer {
        IntCodeComputer {
            program: codes.clone(),
            offset: 0,
            halt: false,
        }
    }

    pub fn is_halt(&self) -> bool {
        self.halt
    }

    pub fn compute(&mut self, mut input: i32) -> i32 {
        let mut result = -1;
        loop {
            let command = self.program[self.offset];
            match command % 100 {
                1 => {
                    let v1 = self.program[self.offset + 1];
                    let v2 = self.program[self.offset + 2];
                    let v3 = self.program[self.offset + 3];
                    self.add(command, v1, v2, v3);
                }
                2 => {
                    let v1 = self.program[self.offset + 1];
                    let v2 = self.program[self.offset + 2];
                    let v3 = self.program[self.offset + 3];
                    self.mul(command, v1, v2, v3);
                }
                3 => {
                    // wait next input
                    if input == -1 {
                        break;
                    }
                    let v1 = self.program[self.offset + 1];
                    self.read(v1, input);
                    input = -1;
                }
                4 => {
                    let v1 = self.program[self.offset + 1];
                    self.write(command, v1, &mut result);
                }
                5 => {
                    let v1 = self.program[self.offset + 1];
                    let v2 = self.program[self.offset + 2];
                    self.jump_if_true(command, v1, v2);
                }
                6 => {
                    let v1 = self.program[self.offset + 1];
                    let v2 = self.program[self.offset + 2];
                    self.jump_if_false(command, v1, v2);
                }
                7 => {
                    let v1 = self.program[self.offset + 1];
                    let v2 = self.program[self.offset + 2];
                    let v3 = self.program[self.offset + 3];
                    self.less_than(command, v1, v2, v3);
                }
                8 => {
                    let v1 = self.program[self.offset + 1];
                    let v2 = self.program[self.offset + 2];
                    let v3 = self.program[self.offset + 3];
                    self.equals(command, v1, v2, v3);
                }
                99 => {
                    self.halt = true;
                    break;
                }
                _ => panic!("Wrong op code.")
            }
        }
        result
    }

    fn add(&mut self, command: i32, v1: i32, v2: i32, v3: i32) {
        self.program[v3 as usize] = self.translate((command % 1000) / 100, v1)
            + self.translate((command % 10000) / 1000, v2);
        self.offset += 4
    }

    fn mul(&mut self, command: i32, v1: i32, v2: i32, v3: i32) {
        self.program[v3 as usize] = self.translate((command % 1000) / 100, v1)
            * self.translate((command % 10000) / 1000, v2);
        self.offset += 4;
    }

    fn write(&mut self, command: i32, v1: i32, result: &mut i32) {
        *result = self.translate((command % 1000) / 100, v1);
        self.offset += 2;
    }

    fn read(&mut self, v1: i32, input: i32) {
        self.program[v1 as usize] = input;
        self.offset += 2;
    }

    fn jump_if_true(&mut self, command: i32, v1: i32, v2: i32) {
        if self.translate((command % 1000) / 100, v1) != 0 {
            self.offset = self.translate((command % 10000) / 1000, v2) as usize
        } else {
            self.offset += 3;
        }
    }

    fn jump_if_false(&mut self, command: i32, v1: i32, v2: i32) {
        if self.translate((command % 1000) / 100, v1) == 0 {
            self.offset = self.translate((command % 10000) / 1000, v2) as usize
        } else {
            self.offset += 3;
        }
    }

    fn less_than(&mut self, command: i32, v1: i32, v2: i32, v3: i32) {
        if self.translate((command % 1000) / 100, v1) < self.translate((command % 10000) / 1000, v2) {
            self.program[v3 as usize] = 1;
        } else {
            self.program[v3 as usize] = 0;
        }
        self.offset += 4;
    }

    fn equals(&mut self, command: i32, v1: i32, v2: i32, v3: i32) {
        if self.translate((command % 1000) / 100, v1) == self.translate((command % 10000) / 1000, v2) {
            self.program[v3 as usize] = 1;
        } else {
            self.program[v3 as usize] = 0;
        }
        self.offset += 4;
    }

    fn translate(&self, mode: i32, v: i32) -> i32 {
        match mode {
            1 => v,
            0 => self.program[v as usize],
            _ => panic!("Wrong mode.")
        }
    }
}



