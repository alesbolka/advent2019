mod input;
use crate::task_05::run_int_code;
use crate::helpers::heap_permutations;

pub fn get_thruster_output(code: &mut Vec<i32>, phase_sequence: Vec<i32>) -> i32 {
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

pub fn task1() {
    let mut max: i32 = 0;
    let code = input::get_puzzle_input();
    for sequence in heap_permutations(&mut vec![0,1,2,3,4], 5, 5) {
        let output = get_thruster_output(&mut code.clone(), sequence.clone());
        if output > max {
            max = output;
        }
    }
    println!("Maximum output: {}", max); // 30940
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
            get_thruster_output(&mut int_code.clone(), vec![1,0,4,3,2])
        );
    }
}
