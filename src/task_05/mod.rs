mod input;

fn run_int_code(instructions: &mut Vec<i32>, inputs: &Vec<i32>) -> Vec<i32> {
    let mut ii = 0;
    let max_ii = instructions.len() - 1;
    let invalid = (max_ii + 5, max_ii + 5);
    let mut results: Vec<i32> = vec![];
    let mut input_index = 0;

    loop {
        if ii > max_ii {
            panic!("No exit code found in intCode");
        }
        let diff = max_ii - ii;
        let opcode = instructions[ii] % 100;

        let first = match diff {
            0 => invalid,
            _ => match instructions[ii] / 100 % 10 {
                0 => (instructions[ii + 1] as usize, instructions[ii + 1] as usize),
                1 => (ii + 1, instructions[ii + 1] as usize),
                _ => invalid,
            },
        };
        let second = match diff {
            0 | 1 => invalid,
            _ => match instructions[ii] / 1000 % 10 {
                0 => (instructions[ii + 2] as usize, instructions[ii + 2] as usize),
                1 => (ii + 2, instructions[ii + 1] as usize),
                _ => invalid,
            },
        };
        let third = match max_ii - ii {
            0 | 1 | 2 => invalid,
            _ => (instructions[ii + 3] as usize, instructions[ii + 3] as usize),
        };

        match opcode {
            // day 2 opcodes
            1 => {
                instructions[third.1] = instructions[first.0] + instructions[second.0];
                ii += 4;
            }
            2 => {
                instructions[third.1] = instructions[first.0] * instructions[second.0];
                ii += 4;
            }
            // part 1 opcodes
            3 => {
                let input = inputs[input_index];
                input_index += 1;
                instructions[first.1] = input;
                println!("Input accepted: {}", instructions[first.1]);
                ii += 2;
            }
            4 => {
                results.push(instructions[first.0]);
                ii += 2;
            }
            // part 2 opcodes
            5 => {
                ii += 3;
                if instructions[first.0] != 0 {
                    ii = instructions[second.0] as usize;
                }
            }
            6 => {
                ii += 3;
                if instructions[first.0] == 0 {
                    ii = instructions[second.0] as usize;
                }
            }
            7 => {
                instructions[third.1] = {
                    if instructions[first.0] < instructions[second.0] {
                        1
                    } else {
                        0
                    }
                };
                ii += 4;
            }
            8 => {
                instructions[third.1] = {
                    if instructions[first.0] == instructions[second.0] {
                        1
                    } else {
                        0
                    }
                };
                ii += 4;
            }
            // day 2 opcodes
            99 => {
                break results;
            }
            _ => panic!("Invalid opcode {} at index {}", opcode, ii),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_compare_8() {
        let mut instructions: Vec<i32> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(&1, run_int_code(&mut instructions, &vec![8]).last().unwrap());
        assert_eq!(&0, run_int_code(&mut instructions, &vec![5]).last().unwrap());
        assert_eq!(&0, run_int_code(&mut instructions, &vec![10]).last().unwrap());
    }

    #[test]
    fn input_compare_8_immediate() {
        let instructions: Vec<i32> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(&1, run_int_code(&mut instructions.clone(), &vec![8]).last().unwrap());
        assert_eq!(&0, run_int_code(&mut instructions.clone(), &vec![5]).last().unwrap());
        assert_eq!(&0, run_int_code(&mut instructions.clone(), &vec![10]).last().unwrap());
    }

    #[test]
    fn input_less_than_8() {
        let instructions: Vec<i32> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(&0, run_int_code(&mut instructions.clone(), &vec![8]).last().unwrap());
        assert_eq!(&1, run_int_code(&mut instructions.clone(), &vec![5]).last().unwrap());
        assert_eq!(&0, run_int_code(&mut instructions.clone(), &vec![10]).last().unwrap());
    }

    #[test]
    fn input_less_than_8_immediate() {
        let instructions: Vec<i32> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(&0, run_int_code(&mut instructions.clone(), &vec![8]).last().unwrap());
        assert_eq!(&1, run_int_code(&mut instructions.clone(), &vec![5]).last().unwrap());
        assert_eq!(&0, run_int_code(&mut instructions.clone(), &vec![10]).last().unwrap());
    }

    #[test]
    fn nonzero_input() {
        let instructions: Vec<i32> = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        assert_eq!(&1, run_int_code(&mut instructions.clone(), &vec![8]).last().unwrap());
        assert_eq!(&0, run_int_code(&mut instructions.clone(), &vec![0]).last().unwrap());
        assert_eq!(&1, run_int_code(&mut instructions.clone(), &vec![-1]).last().unwrap());
    }

    #[test]
    fn nonzero_input_immediate() {
        let instructions: Vec<i32> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(&1, run_int_code(&mut instructions.clone(), &vec![8]).last().unwrap());
        assert_eq!(&0, run_int_code(&mut instructions.clone(), &vec![0]).last().unwrap());
        assert_eq!(&1, run_int_code(&mut instructions.clone(), &vec![-1]).last().unwrap());
    }

    #[test]
    fn compare_to_8_advanced() {
        let instructions: Vec<i32> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
            1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
            1105, 1, 46, 98, 99,
        ];
        assert_eq!(&1000, run_int_code(&mut instructions.clone(), &vec![8]).last().unwrap());
        assert_eq!(&999, run_int_code(&mut instructions.clone(), &vec![0]).last().unwrap());
        assert_eq!(&1001, run_int_code(&mut instructions.clone(), &vec![10]).last().unwrap());
    }

    #[test]
    pub fn compare_day_1() {
        let mut instructions: Vec<i32> = vec![
            1, 95, 7, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 19, 5, 23, 2, 13, 23, 27,
            1, 10, 27, 31, 2, 6, 31, 35, 1, 9, 35, 39, 2, 10, 39, 43, 1, 43, 9, 47, 1, 47, 9, 51, 2,
            10, 51, 55, 1, 55, 9, 59, 1, 59, 5, 63, 1, 63, 6, 67, 2, 6, 67, 71, 2, 10, 71, 75, 1, 75,
            5, 79, 1, 9, 79, 83, 2, 83, 10, 87, 1, 87, 6, 91, 1, 13, 91, 95, 2, 10, 95, 99, 1, 99, 6,
            103, 2, 13, 103, 107, 1, 107, 2, 111, 1, 111, 9, 0, 99, 2, 14, 0, 0,
        ];
        run_int_code(&mut instructions, &vec![]);

        let expected: Vec<i32> = vec![
            19690720, 95, 7, 2, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 190, 1, 19, 5, 191, 2, 13,
            23, 955, 1, 10, 27, 959, 2, 6, 31, 1918, 1, 9, 35, 1921, 2, 10, 39, 7684, 1, 43, 9, 7687,
            1, 47, 9, 7690, 2, 10, 51, 30760, 1, 55, 9, 30763, 1, 59, 5, 30764, 1, 63, 6, 30766, 2, 6,
            67, 61532, 2, 10, 71, 246128, 1, 75, 5, 246129, 1, 9, 79, 246132, 2, 83, 10, 984528, 1, 87,
            6, 984530, 1, 13, 91, 984535, 2, 10, 95, 3938140, 1, 99, 6, 3938142, 2, 13, 103, 19690710,
            1, 107, 2, 19690717, 1, 111, 9, 0, 99, 2, 14, 0, 0,
        ];
        assert_eq!(instructions, expected);
    }
}




pub fn part1() {
    let mut instructions: Vec<i32> = input::get_input();

    let res = run_int_code(&mut instructions, &vec![]);
    println!("result: {:?}", res);
}

pub fn part2() {
    let mut instructions: Vec<i32> = input::get_input();

    let res = run_int_code(&mut instructions, &vec![5]);
    println!("result: {:?}", res); // 11460760
}
