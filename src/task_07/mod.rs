mod input;
use crate::helpers::heap_permutations;
use crate::task_05::{run_int_code, Machine};

pub fn get_thruster_output(code: &Vec<i32>, phase_sequence: Vec<i32>) -> i32 {
    let mut carry: i32 = 0;
    for &phase in phase_sequence.iter() {
        let out = run_int_code(&mut code.clone(), &vec![phase, carry]);
        carry = match out.last() {
            Some(&x) => x,
            _ => panic!("Invalid output {:?}", out),
        };
    }
    carry
}

pub fn get_thruster_recursive(code: &Vec<i32>, phase_sequence: Vec<i32>) -> i32 {
    let mut out: i32 = 0;
    let mut machines: Vec<Machine> = phase_sequence
        .iter()
        .map(|_xx| Machine::new(&code, &vec![]))
        .collect();

    for (ii, &phase) in phase_sequence.iter().enumerate() {
        machines[ii].set_input(&vec![phase, out]);
        let res = machines[ii].run();
        out = match res {
            Some(xx) => xx,
            _ => panic!("Invalid output {:?}", out),
        };
    }

    let mut done = false;
    while !done {
        done = true;
        for ii in 0..machines.len() {
            machines[ii].set_input(&vec![out]);
            match machines[ii].run() {
                Some(xx) => {
                    out = xx;
                    done = false;
                }
                None => (),
            }
        }
    }

    out
}

pub fn task1() {
    let mut max: i32 = 0;
    let code = input::get_puzzle_input();
    for sequence in heap_permutations(&mut vec![0, 1, 2, 3, 4], 5, 5) {
        let output = get_thruster_output(&mut code.clone(), sequence.clone());
        if output > max {
            max = output;
        }
    }
    println!("Maximum output: {}", max); // 30940
}

pub fn task2() {
    let mut max: i32 = 0;
    let code = input::get_puzzle_input();
    for sequence in heap_permutations(&mut vec![9, 8, 7, 6, 5], 5, 5) {
        let res = get_thruster_recursive(&code, sequence);
        if res > max {
            max = res;
        }
    }
    println!("Maximum output: {}", max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_test_1() {
        let int_code = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(
            43210,
            get_thruster_output(&mut int_code.clone(), vec![4, 3, 2, 1, 0])
        );
    }

    #[test]
    fn sequence_test_2() {
        let int_code = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(
            54321,
            get_thruster_output(&mut int_code.clone(), vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn sequence_test_3() {
        let int_code = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(
            65210,
            get_thruster_output(&mut int_code.clone(), vec![1, 0, 4, 3, 2])
        );
    }

    #[test]
    fn recursive_test_1() {
        let int_code = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(
            139629729,
            get_thruster_recursive(&int_code, vec![9, 8, 7, 6, 5])
        );
    }

    #[test]
    fn recursive_test_2() {
        let int_code = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(
            18216,
            get_thruster_recursive(&int_code, vec![9, 7, 8, 5, 6])
        );
    }
}
