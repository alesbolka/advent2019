mod input;

pub struct Machine {
    code: Vec<i64>,
    code_raw: Vec<i64>,
    pub halted: bool,
    ii: usize,
    input: Vec<i64>,
    input_ii: usize,
    max_ii: usize,
    rel_offset: i64,
    read: [usize; 3],
    write: [usize; 3],
}

impl Machine {
    pub fn new(code: &Vec<i64>, inp: &Vec<i64>) -> Machine {
        Machine {
            code: code.clone(),
            code_raw: code.clone(),
            halted: false,
            ii: 0,
            input: inp.clone(),
            input_ii: 0,
            max_ii: code.len() - 1,
            rel_offset: 0,
            read: [0; 3],
            write: [0; 3],
        }
    }
}

impl Machine {
    pub fn reset(&mut self) {
        self.code = self.code_raw.clone();
        self.halted = false;
        self.ii = 0;
    }

    fn check_memory(&mut self, min: usize) {
        if min > self.max_ii {
            self.extend_memory(min);
        }
    }

    fn extend_memory(&mut self, to: usize) {
        self.code.append(&mut vec![0; to - self.code.len() + 1]);
        self.max_ii = self.code.len() - 1;
    }

    pub fn run(&mut self) -> Option<i64> {
        if self.halted {
            panic!("Called while halted");
        }
        loop {
            self.check_memory(self.ii + 4);
            self.get_params();

            match self.code[self.ii] % 100 {
                1 => self.opcode1(),
                2 => self.opcode2(),
                3 => self.opcode3(),
                4 => return Some(self.opcode4()),
                5 => self.opcode5(),
                6 => self.opcode6(),
                7 => self.opcode7(),
                8 => self.opcode8(),
                9 => self.opcode9(),
                99 => {
                    self.halted = true;
                    return None;
                }
                _ => panic!("Invalid opcode {} at index {}", self.code[self.ii], self.ii),
            }
        }
    }

    pub fn full_run(&mut self) -> Vec<i64> {
        let mut out = vec![];
        loop {
            match self.run() {
                Some(x) => out.push(x),
                None => break,
            }
        };

        out
    }

    pub fn set_input(&mut self, input: &Vec<i64>) {
        self.input = input.clone();
        self.input_ii = 0;
    }

    fn next_input(&mut self) -> i64 {
        self.input_ii += 1;
        self.input[self.input_ii - 1]
    }

    fn get_params(&mut self) {
        let opcode = self.code[self.ii];
        let modes: [i64; 3] = [
            (opcode / 100) % 10,
            (opcode / 1000) % 10,
            (opcode / 10000) % 10,
        ];

        for offset in 0..3 {
            let index = self.ii + offset + 1;
            let immediate = index;
            let positional = (self.code[index]) as usize;
            let relative = (self.code[index] + self.rel_offset) as usize;

            match modes[offset] {
                0 => {
                    self.read[offset] = positional;
                    self.write[offset] = positional;
                },
                1 => {
                    self.read[offset] = immediate;
                    self.write[offset] = positional;
                },
                2 => {
                    self.read[offset] = relative;
                    self.write[offset] = relative;
                },
                _ => panic!("invalid mode {} at {}", modes[offset], self.ii),
            };
        }
        match opcode % 100 {
            1 | 2 | 7 | 8 => {
                self.check_memory(self.read[0]);
                self.check_memory(self.read[1]);
                self.check_memory(self.write[2]);
            },
            3 => self.check_memory(self.write[1]),
            4 => self.check_memory(self.read[1]),
            5 | 6 => {
                self.check_memory(self.read[0]);
                self.check_memory(self.read[1]);
            },
            9 => self.check_memory(self.read[0]),
            99 => {},
            _ => panic!("Invalid opcode {} at {}", opcode, self.ii),
        };
    }

    fn opcode1(&mut self) {
        self.code[self.write[2]] = self.code[self.read[0]] + self.code[self.read[1]];
        self.ii += 4;
    }

    fn opcode2(&mut self) {
        self.code[self.write[2]] = self.code[self.read[0]] * self.code[self.read[1]];
        self.ii += 4;
    }

    fn opcode3(&mut self) {
        self.code[self.write[0]] = self.next_input();
        self.ii += 2;
    }

    fn opcode4(&mut self) -> i64 {
        let res = self.code[self.read[0]];
        self.ii += 2;

        res
    }

    fn opcode5(&mut self) {
        if self.code[self.read[0]] != 0 {
            self.ii = self.code[self.read[1]] as usize;
        } else {
            self.ii += 3;
        }
    }

    fn opcode6(&mut self) {
        if self.code[self.read[0]] == 0 {
            self.ii = self.code[self.read[1]] as usize;
        } else {
            self.ii += 3;
        }
    }

    fn opcode7(&mut self) {
        if self.code[self.read[0]] < self.code[self.read[1]] {
            self.code[self.write[2]] = 1
        } else {
            self.code[self.write[2]] = 0;
        }
        self.ii += 4;
    }

    fn opcode8(&mut self) {
        if self.code[self.read[0]] == self.code[self.read[1]] {
            self.code[self.write[2]] = 1
        } else {
            self.code[self.write[2]] = 0;
        }
        self.ii += 4;
    }

    fn opcode9(&mut self) {
        self.rel_offset += self.code[self.read[0]];
        self.ii += 2;
    }
}

pub fn part1() {
    let instructions: Vec<i64> = input::get_input();
    let mut mac = Machine::new(&instructions, &vec![1]);

    let res = mac.full_run();
    println!("{:?}", res);
}

// pub fn part2() {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let code = vec![1, 1, 2, 5, 99, 0];
        let mut mac = Machine::new(&code, &vec![]);
        mac.run();
        assert_eq!(3, mac.code[5]);
    }

    #[test]
    fn addition_i() {
        let code = vec![101, 5, 2, 5, 99, 0];
        let mut mac = Machine::new(&code, &vec![]);
        mac.run();
        assert_eq!(7, mac.code[5]);
    }

    #[test]
    fn addition_ii() {
        let code = vec![1101, 3, 2, 5, 99, 0];
        let mut mac = Machine::new(&code, &vec![]);
        mac.run();
        assert_eq!(5, mac.code[5]);
    }

    #[test]
    fn addition_r() {
        let code = vec![201, 1, 2, 5, 99, 0];
        let mut mac = Machine::new(&code, &vec![]);
        mac.rel_offset = 2;
        mac.run();
        assert_eq!(7, mac.code[5]);
    }

    #[test]
    fn addition_rr() {
        let code = vec![2201, 1, 2, 5, 99, 0];
        let mut mac = Machine::new(&code, &vec![]);
        mac.rel_offset = 3;
        mac.run();
        assert_eq!(99, mac.code[5]);
    }

    #[test]
    fn multiplication() {
        let code = vec![2, 3, 2, 5, 99, 0];
        let mut mac = Machine::new(&code, &vec![]);
        mac.run();
        assert_eq!(10, mac.code[5]);
    }

    #[test]
    fn multiplication_i() {
        let code = vec![1002, 2, 4, 5, 99, 0];
        let mut mac = Machine::new(&code, &vec![]);
        mac.run();
        assert_eq!(16, mac.code[5]);
    }

    #[test]
    fn multiplication_ii() {
        let code = vec![1102, 3, 2, 5, 99, 0];
        let mut mac = Machine::new(&code, &vec![]);
        mac.run();
        assert_eq!(6, mac.code[5]);
    }

    #[test]
    fn multiplication_r() {
        let code = vec![202, 0, 0, 5, 99, 0];
        let mut mac = Machine::new(&code, &vec![]);
        mac.rel_offset = 3;
        mac.run();
        assert_eq!(1010, mac.code[5]);
    }

    #[test]
    fn read_input() {
        let code = vec![3, 0, 99];
        let mut mac = Machine::new(&code, &vec![1]);
        mac.run();
        assert_eq!(1, mac.code[0]);
    }

    #[test]
    fn output() {
        let code = vec![4, 0, 4, 4, 99];
        let mut mac = Machine::new(&code, &vec![4,8]);
        assert_eq!(Some(4), mac.run());
        assert_eq!(Some(99), mac.run());
        assert_eq!(None, mac.run());
        assert_eq!(true, mac.halted);
    }

    #[test]
    fn output_ir() {
        let code = vec![104, 1, 204, 4, 99];
        let mut mac = Machine::new(&code, &vec![]);
        mac.rel_offset = -2;
        assert_eq!(Some(1), mac.run());
        assert_eq!(Some(204), mac.run());
        assert_eq!(None, mac.run());
        assert_eq!(true, mac.halted);
    }

    #[test]
    fn not0() {
        let code = vec![5, 1, 5, 0, 99, 4];
        let mut mac = Machine::new(&code, &vec![]);
        mac.run(); // will panic if incorrect
        let code = vec![5, 4, 4, 99, 0];
        let mut mac = Machine::new(&code, &vec![]);
        mac.run();
    }

    #[test]
    fn not0_i() {
        let code = vec![105, 1, 5, 0, 99, 4];
        let mut mac = Machine::new(&code, &vec![]);
        mac.run();
        let code = vec![105, 0, 4, 99, 0];
        let mut mac = Machine::new(&code, &vec![]);
        mac.run();
    }

    #[test]
    fn example_9_01() {
        let code = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut mac = Machine::new(&code, &vec![]);
        let mut out = vec![];
        loop {
            match mac.run() {
                Some(x) => out.push(x),
                None => break,
            }
        };

        assert_eq!(code, out);
    }

    #[test]
    fn example_9_02() {
        let code = vec![1102,34915192,34915192,7,4,7,99,0];
        let mut mac = Machine::new(&code, &vec![]);
        let out = mac.full_run();

        assert_eq!(16, format!("{}", out.last().unwrap()).len());
    }

    #[test]
    fn example_9_03() {
        let code = vec![104,1125899906842624,99];
        let mut mac = Machine::new(&code, &vec![]);
        let out = mac.full_run();

        assert_eq!(&code[1], out.last().unwrap());
    }
}
