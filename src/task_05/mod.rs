use std::io;
mod input;

fn run_int_code(instructions: &mut Vec<i32>) -> Vec<i32> {
    let mut ii = 0;
    let max_ii = instructions.len() - 1;
    let invalid = (max_ii + 5, max_ii + 5);
    let mut first_input = false;
    let mut results: Vec<i32> = vec![];

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
            _ => match instructions[ii] / 10000 % 10 {
                0 => (instructions[ii + 3] as usize, instructions[ii + 3] as usize),
                1 => (ii + 3, instructions[ii + 3] as usize),
                _ => invalid,
            },
        };

        match opcode {
            1 => {
                instructions[third.1] = instructions[first.0] + instructions[second.0];
                ii += 4;
            }
            2 => {
                instructions[third.1] = instructions[first.0] * instructions[second.0];
                ii += 4;
            }
            3 => {
                let input = loop {
                    if !first_input {
                        first_input = true;
                        break 1;
                    }
                    println!("Input number");
                    let mut guess = String::new();

                    io::stdin()
                        .read_line(&mut guess)
                        .expect("Failed to read line");

                    match guess.trim().parse::<i32>() {
                        Ok(x) => break x,
                        Err(_e) => println!("{} is not a number", guess.trim()),
                    }
                };
                instructions[first.1] = input;
                ii += 2;
            }
            4 => {
                results.push(instructions[first.0]);
                ii += 2;
            },
            99 => {
                break results;
            },
            _ => panic!("Invalid opcode {} at index {}", opcode, ii),
        }
    }
}

pub fn old_tests() {
    let mut instructions: Vec<i32> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    run_int_code(&mut instructions);
    let expected: Vec<i32> = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(instructions, expected);
    let mut instructions: Vec<i32> = vec![1, 0, 0, 0, 99];
    run_int_code(&mut instructions);
    let expected: Vec<i32> = vec![2, 0, 0, 0, 99];
    assert_eq!(instructions, expected);
    let mut instructions: Vec<i32> = vec![2, 3, 0, 3, 99];
    run_int_code(&mut instructions);
    let expected: Vec<i32> = vec![2, 3, 0, 6, 99];
    assert_eq!(instructions, expected);
    let mut instructions: Vec<i32> = vec![2, 4, 4, 5, 99, 0];
    run_int_code(&mut instructions);
    let expected: Vec<i32> = vec![2, 4, 4, 5, 99, 9801];
    assert_eq!(instructions, expected);
    let mut instructions: Vec<i32> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    run_int_code(&mut instructions);
    let expected: Vec<i32> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
    assert_eq!(instructions, expected);
}

pub fn test() {
    let mut instructions: Vec<i32> = vec![1101,-1,7,7,4,7,99,11,0,99];
    run_int_code(&mut instructions);
    println!("{:?}", instructions[7]);
}

pub fn compare_day_1() {
    let mut instructions: Vec<i32> = vec![
        1, 95, 7, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 19, 5, 23, 2, 13, 23, 27,
        1, 10, 27, 31, 2, 6, 31, 35, 1, 9, 35, 39, 2, 10, 39, 43, 1, 43, 9, 47, 1, 47, 9, 51, 2,
        10, 51, 55, 1, 55, 9, 59, 1, 59, 5, 63, 1, 63, 6, 67, 2, 6, 67, 71, 2, 10, 71, 75, 1, 75,
        5, 79, 1, 9, 79, 83, 2, 83, 10, 87, 1, 87, 6, 91, 1, 13, 91, 95, 2, 10, 95, 99, 1, 99, 6,
        103, 2, 13, 103, 107, 1, 107, 2, 111, 1, 111, 9, 0, 99, 2, 14, 0, 0,
    ];
    run_int_code(&mut instructions);

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

pub fn part1() {
    let mut instructions: Vec<i32> = input::get_input();

    let res = run_int_code(&mut instructions);
    println!("result: {:?}", res);
}
