fn run_int_code(instructions: &mut Vec<i32>) -> (u8, i32) {
    let mut ii = 0;
    let max_ii = instructions.len() - 1;
    let invalid = (max_ii + 5, max_ii + 5);
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
                println!("{:?}, adding up {} and {} into {}", (1, first, second, third), instructions[first.0], instructions[second.0], third.1);
                instructions[third.1] = instructions[first.0] + instructions[second.0];
                ii += 4;
            },
            2 => {
                println!("{:?}, multiplying {} and {} into {}", (2, first, second, third), instructions[first.0], instructions[second.0], third.1);
                instructions[third.1] = instructions[first.0] * instructions[second.0];
                ii += 4;
            },
            3 => {
                println!("{:?}, writing {} into {}", (3, first, second, third), instructions[first.1], instructions[first.0]);
                instructions[first.1] = instructions[first.0];
                ii += 2;
            },
            4 => break (0, instructions[first.0]),
            99 => break (1, 0),
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
    let mut instructions: Vec<i32> = vec![
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1101, 37, 34, 224, 101, -71, 224, 224, 4,
        224, 1002, 223, 8, 223, 101, 6, 224, 224, 1, 224, 223, 223, 1002, 113, 50, 224, 1001, 224,
        -2550, 224, 4, 224, 1002, 223, 8, 223, 101, 2, 224, 224, 1, 223, 224, 223, 1101, 13, 50,
        225, 102, 7, 187, 224, 1001, 224, -224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 5, 224,
        1, 224, 223, 223, 1101, 79, 72, 225, 1101, 42, 42, 225, 1102, 46, 76, 224, 101, -3496, 224,
        224, 4, 224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1102, 51, 90, 225, 1101,
        11, 91, 225, 1001, 118, 49, 224, 1001, 224, -140, 224, 4, 224, 102, 8, 223, 223, 101, 5,
        224, 224, 1, 224, 223, 223, 2, 191, 87, 224, 1001, 224, -1218, 224, 4, 224, 1002, 223, 8,
        223, 101, 4, 224, 224, 1, 224, 223, 223, 1, 217, 83, 224, 1001, 224, -124, 224, 4, 224,
        1002, 223, 8, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1101, 32, 77, 225, 1101, 29, 80,
        225, 101, 93, 58, 224, 1001, 224, -143, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 4, 224,
        1, 223, 224, 223, 1101, 45, 69, 225, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105,
        1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274,
        1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0,
        1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0,
        1105, 1, 99999, 7, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 329, 101, 1, 223, 223, 108,
        677, 226, 224, 102, 2, 223, 223, 1005, 224, 344, 1001, 223, 1, 223, 1108, 226, 677, 224,
        102, 2, 223, 223, 1005, 224, 359, 1001, 223, 1, 223, 8, 677, 226, 224, 102, 2, 223, 223,
        1006, 224, 374, 1001, 223, 1, 223, 107, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 389,
        101, 1, 223, 223, 1108, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 404, 1001, 223, 1,
        223, 108, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 419, 101, 1, 223, 223, 7, 226, 677,
        224, 1002, 223, 2, 223, 1006, 224, 434, 1001, 223, 1, 223, 107, 226, 677, 224, 102, 2, 223,
        223, 1005, 224, 449, 101, 1, 223, 223, 1108, 677, 677, 224, 1002, 223, 2, 223, 1006, 224,
        464, 101, 1, 223, 223, 7, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 479, 101, 1, 223,
        223, 1007, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 494, 101, 1, 223, 223, 1008, 226,
        226, 224, 102, 2, 223, 223, 1006, 224, 509, 1001, 223, 1, 223, 107, 677, 677, 224, 102, 2,
        223, 223, 1006, 224, 524, 1001, 223, 1, 223, 8, 226, 226, 224, 1002, 223, 2, 223, 1005,
        224, 539, 1001, 223, 1, 223, 1007, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 554, 1001,
        223, 1, 223, 1007, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 569, 1001, 223, 1, 223, 8,
        226, 677, 224, 1002, 223, 2, 223, 1006, 224, 584, 101, 1, 223, 223, 108, 226, 226, 224,
        1002, 223, 2, 223, 1006, 224, 599, 101, 1, 223, 223, 1107, 677, 226, 224, 1002, 223, 2,
        223, 1005, 224, 614, 1001, 223, 1, 223, 1107, 226, 677, 224, 102, 2, 223, 223, 1006, 224,
        629, 1001, 223, 1, 223, 1008, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 644, 101, 1, 223,
        223, 1107, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 659, 1001, 223, 1, 223, 1008, 677,
        677, 224, 102, 2, 223, 223, 1006, 224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
    ];

    let (sta, res) = run_int_code(&mut instructions);
    assert_eq!(sta, 0);
    println!("Status code: {}, result: {}", sta, res);
}
