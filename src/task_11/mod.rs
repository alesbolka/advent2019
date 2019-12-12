mod input;
use super::task_09::Machine;
use std::collections::HashMap;
const BLOCK_WHITE: char = '\u{2588}';
const BLOCK_BLACK: char = ' ';


fn rotate(current: u8, dir: u8) -> u8 {
    match dir {
        0 => (4 + current - 1) % 4,
        1 => (current + 1) % 4,
        _ => panic!("Invalid direction {}", dir),
    }
}

fn travel(current: (i64, i64), dir: u8) -> (i64, i64) {
    match dir {
        0 => (current.0, current.1 + 1),
        1 => (current.0 + 1, current.1),
        2 => (current.0, current.1 - 1),
        3 => (current.0 - 1, current.1),
        _ => panic!("Invalid orientation {}", dir),
    }
}

fn count_painted_panels(mac: &mut Machine) -> usize {
    // 0 = up
    // 1 = right
    // 2 = down
    // 3 = left
    let mut dir = 0;
    let mut map: HashMap<(i64, i64), char> = HashMap::new();
    let mut coords = (0,0);
    loop {
        let mut standing_on = 0;
        if *map.get(&coords).unwrap_or(&BLOCK_BLACK) != BLOCK_BLACK {
            standing_on = 1;
        }

        mac.set_input(&vec![standing_on]);

        match mac.run() {
            Some(0) => {
                // println!("Painting {:?} black", coords);
                map.insert(coords, BLOCK_BLACK);
            },
            Some(1) => {
                // println!("Painting {:?} white", coords);
                map.insert(coords, BLOCK_WHITE);
            },
            None => break,
            Some(x) => panic!("Invalid output value {}", x),
        };
        match mac.run() {
            Some(0) => {
                // println!("Turning left, moving to {:?}", coords);
                dir = rotate(dir, 0);
                coords = travel(coords, dir);
            },
            Some(1) => {
                // println!("Turning right, moving to {:?}", coords);
                dir = rotate(dir, 1);
                coords = travel(coords, dir);
            },
            None => break,
            Some(x) => panic!("Invalid output value {}", x),
        };
    }
    map.len()
}

pub fn part1() {

    let mut mac = Machine::new(&input::get_code(), &vec![]);
    let count = count_painted_panels(&mut mac);


    println!("Painted {} tiles", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation () {
        let mut dir = 0;
        dir = rotate(dir, 1);
        assert_eq!(1, dir);
        dir = rotate(dir, 0);
        assert_eq!(0, dir);
        dir = rotate(dir, 0);
        assert_eq!(3, dir);
        dir = rotate(dir, 1);
        assert_eq!(0, dir);
    }

    #[test]
    fn travel_test () {
        let mut xx = (0,0);
        xx = travel(xx, 0);
        assert_eq!(xx, (0,1));
        xx = travel(xx, 0);
        assert_eq!(xx, (0,2));
        xx = travel(xx, 3);
        assert_eq!(xx, (-1,2));
        xx = travel(xx, 2);
        assert_eq!(xx, (-1,1));
        xx = travel(xx, 1);
        assert_eq!(xx, (0,1));
    }

    #[test]
    fn travel_test2 () {
    }
}
