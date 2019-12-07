pub fn is_valid(mut num: u32) -> bool {
    let mut repeated = false;
    let mut prev: u8 = 200;

    for _ii in 0..6 {
        let digit = (num % 10) as u8;
        if prev == digit {
            repeated = true;
        }
        if prev < digit {
            return false;
        }
        prev = digit;
        num /= 10;
    };

    repeated
}

pub fn is_valid2(mut num: u32) -> bool {
    let mut prev: u8 = 200;
    let mut sequence_lengths: Vec<u8> = vec!{};
    let mut current_sequence = 1;

    for ii in 0..6 {
        let digit = (num % 10) as u8;
        if ii > 0 {
            if prev < digit {
                return false;
            }

            if prev == digit {
                current_sequence += 1;
            } else if current_sequence > 1 {
                sequence_lengths.push(current_sequence);
                current_sequence = 1;
            }
        }

        prev = digit;
        num /= 10;
    };

    if current_sequence > 1 {
        sequence_lengths.push(current_sequence);
    }

    match sequence_lengths.iter().min() {
        Some(2) => true,
        _ => false,
    }
}

pub fn ex1() {
    assert!(is_valid(111111));
    assert!(!is_valid(223450));
    assert!(!is_valid(123789));
    println!("Tests passed");
}

pub fn ex2() {
    assert!(is_valid2(112233));
    assert!(!is_valid2(123444));
    assert!(is_valid2(111122));
    assert!(!is_valid2(111111));
    assert!(!is_valid2(112211));
    println!("Tests passed");
}

pub fn task1() {
    let min = 347312 as u32;
    let max = 805915 as u32;
    let mut counter = 0;
    for ii in min..(max+1) {
        if is_valid(ii) {
            counter += 1;
        }
    }
    println!("{}", counter);
}

pub fn task2() {
    let min = 347312 as u32;
    let max = 805915 as u32;
    let mut counter = 0;
    for ii in min..(max+1) {
        if is_valid2(ii) {
            counter += 1;
        }
    }
    println!("{}", counter);
}
