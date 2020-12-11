use std::collections::BTreeMap;

pub struct IntCodeComputer23 {
    program: BTreeMap<i64, i64>,
    offset: i64,
    halt: bool,
    relative_base: i64,
}

//noinspection ALL
impl IntCodeComputer23 {
    pub fn new(codes: &Vec<i64>) -> IntCodeComputer23 {
        let mut program: BTreeMap<i64, i64> = BTreeMap::new();
        for (i, c) in codes.iter().enumerate() {
            program.insert(i as i64, *c);
        }

        IntCodeComputer23 {
            program,
            offset: 0,
            halt: false,
            relative_base: 0,
        }
    }

    pub fn is_halt(&self) -> bool {
        self.halt
    }

    pub fn compute(&mut self, input: &mut Vec<i64>) -> Vec<i64> {
        let mut result = vec![];
        while !self.halt {
            let command = self.program.get(&self.offset).unwrap().clone();
            match command % 100 {
                1 => {
                    let v1 = self.program.get(&(self.offset + 1)).unwrap_or(&(0 as i64)).clone();
                    let v2 = self.program.get(&(self.offset + 2)).unwrap_or(&(0 as i64)).clone();
                    let v3 = self.program.get(&(self.offset + 3)).unwrap_or(&(0 as i64)).clone();
                    self.add(command, v1, v2, v3);
                }
                2 => {
                    let v1 = self.program.get(&(self.offset + 1)).unwrap_or(&(0 as i64)).clone();
                    let v2 = self.program.get(&(self.offset + 2)).unwrap_or(&(0 as i64)).clone();
                    let v3 = self.program.get(&(self.offset + 3)).unwrap_or(&(0 as i64)).clone();
                    self.mul(command, v1, v2, v3);
                }
                3 => {
                    let v1 = self.program.get(&(self.offset + 1)).unwrap_or(&(0 as i64)).clone();
                    if input.is_empty() {
                        break;
                    }
                    self.read(command, v1, input.remove(0));
                }
                4 => {
                    let v1 = self.program.get(&(self.offset + 1)).unwrap_or(&(0 as i64)).clone();
                    self.write(command, v1, &mut result);
                }
                5 => {
                    let v1 = self.program.get(&(self.offset + 1)).unwrap_or(&(0 as i64)).clone();
                    let v2 = self.program.get(&(self.offset + 2)).unwrap_or(&(0 as i64)).clone();
                    self.jump_if_true(command, v1, v2);
                }
                6 => {
                    let v1 = self.program.get(&(self.offset + 1)).unwrap_or(&(0 as i64)).clone();
                    let v2 = self.program.get(&(self.offset + 2)).unwrap_or(&(0 as i64)).clone();
                    self.jump_if_false(command, v1, v2);
                }
                7 => {
                    let v1 = self.program.get(&(self.offset + 1)).unwrap_or(&(0 as i64)).clone();
                    let v2 = self.program.get(&(self.offset + 2)).unwrap_or(&(0 as i64)).clone();
                    let v3 = self.program.get(&(self.offset + 3)).unwrap_or(&(0 as i64)).clone();
                    self.less_than(command, v1, v2, v3);
                }
                8 => {
                    let v1 = self.program.get(&(self.offset + 1)).unwrap_or(&(0 as i64)).clone();
                    let v2 = self.program.get(&(self.offset + 2)).unwrap_or(&(0 as i64)).clone();
                    let v3 = self.program.get(&(self.offset + 3)).unwrap_or(&(0 as i64)).clone();
                    self.equals(command, v1, v2, v3);
                }
                9 => {
                    let v1 = self.program.get(&(self.offset + 1)).unwrap_or(&(0 as i64)).clone();
                    self.adj_relative_base(command, v1);
                }
                99 => {
                    self.halt = true;
                }
                _ => panic!("Wrong op code.")
            }
        }
        result
    }

    fn add(&mut self, command: i64, v1: i64, v2: i64, v3: i64) {
        let i = self.get_value((command % 1000) / 100, v1);
        let i1 = self.get_value((command % 10000) / 1000, v2);
        let pos = self.to_pos((command % 100000) / 10000, v3);
        self.program.insert(pos, i + i1);
        self.offset += 4
    }

    fn mul(&mut self, command: i64, v1: i64, v2: i64, v3: i64) {
        let i = self.get_value((command % 1000) / 100, v1);
        let i1 = self.get_value((command % 10000) / 1000, v2);
        let pos = self.to_pos((command % 100000) / 10000, v3);
        self.program.insert(pos, i * i1);
        self.offset += 4;
    }

    fn write(&mut self, command: i64, v1: i64, result: &mut Vec<i64>) {
        result.push(self.get_value((command % 1000) / 100, v1));
        self.offset += 2;
    }

    fn read(&mut self, command: i64, v1: i64, input: i64) {
        let pos = self.to_pos((command % 1000) / 100, v1);
        self.program.insert(pos, input);
        self.offset += 2;
    }

    fn jump_if_true(&mut self, command: i64, v1: i64, v2: i64) {
        if self.get_value((command % 1000) / 100, v1) != 0 {
            self.offset = self.get_value((command % 10000) / 1000, v2)
        } else {
            self.offset += 3;
        }
    }

    fn jump_if_false(&mut self, command: i64, v1: i64, v2: i64) {
        if self.get_value((command % 1000) / 100, v1) == 0 {
            self.offset = self.get_value((command % 10000) / 1000, v2)
        } else {
            self.offset += 3;
        }
    }

    fn less_than(&mut self, command: i64, v1: i64, v2: i64, v3: i64) {
        let pos = self.to_pos((command % 100000) / 10000, v3);
        if self.get_value((command % 1000) / 100, v1) < self.get_value((command % 10000) / 1000, v2) {
            self.program.insert(pos, 1);
        } else {
            self.program.insert(pos, 0);
        }
        self.offset += 4;
    }

    fn equals(&mut self, command: i64, v1: i64, v2: i64, v3: i64) {
        let pos = self.to_pos((command % 100000) / 10000, v3);
        if self.get_value((command % 1000) / 100, v1) == self.get_value((command % 10000) / 1000, v2) {
            self.program.insert(pos, 1);
        } else {
            self.program.insert(pos, 0);
        }
        self.offset += 4;
    }

    fn adj_relative_base(&mut self, command: i64, v1: i64) {
        self.relative_base += self.get_value((command % 1000) / 100, v1);
        self.offset += 2;
    }

    fn get_value(&mut self, mode: i64, v: i64) -> i64 {
        match mode {
            2 => self.program.get(&(self.relative_base + v)).unwrap_or(&(0 as i64)).clone(),
            1 => v,
            0 => self.program.get(&v).unwrap_or(&(0 as i64)).clone(),
            _ => panic!("Wrong mode.")
        }
    }

    fn to_pos(&self, mode: i64, v: i64) -> i64 {
        match mode {
            2 => self.relative_base + v,
            0 => v,
            _ => panic!("Wrong mode.")
        }
    }
}



