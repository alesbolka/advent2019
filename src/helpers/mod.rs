use std::cmp::PartialEq;
use std::fmt::Debug;

pub fn equal_vectors<T: PartialEq + Debug>(a: &Vec<T>, b: &Vec<T>) -> bool {
    if a.len() != b.len() {
        println!("not equal len");
        return false;
    }
    for xx in a.iter() {
        if !b.contains(xx) {
            println!("{:?} not in {:?}", xx, b);
            return false;
        }
    }
    true
}

pub fn heap_permutations(work: &mut Vec<i32>, size: usize, n: usize) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = vec![];

    if size == 1 {
        out.push(work.clone());
    }

    for ii in 0..size {
        out.append(&mut heap_permutations(work, size - 1, n));

        match size % 2 {
            1 => {
                let temp = work[0];
                work[0] = work[size - 1];
                work[size - 1] = temp;
            }
            _ => {
                let temp = work[ii];
                work[ii] = work[size - 1];
                work[size - 1] = temp;
            }
        };
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_generator_test() {
        assert!(equal_vectors(
            &vec![vec![1, 2], vec![2, 1]],
            &heap_permutations(&mut vec![1, 2], 2, 2)
        ));
        let tri = &heap_permutations(&mut vec![1, 2, 3], 3, 3);
        assert!(
            equal_vectors(
                &vec![
                    vec![1, 2, 3],
                    vec![1, 3, 2],
                    vec![2, 1, 3],
                    vec![2, 3, 1],
                    vec![3, 1, 2],
                    vec![3, 2, 1]
                ],
                &tri,
            ),
            "Invalid permutation array {:?}",
            tri
        );
    }
}
