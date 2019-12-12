mod input;
use super::task_09::Machine;
use std::collections::HashMap;
use std::cmp::{min, max};
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

fn paint(mac: &mut Machine, starting: char) -> (HashMap<(i64, i64), char>, (i64, i64), (i64, i64)) {
    // 0 = up
    // 1 = right
    // 2 = down
    // 3 = left
    let mut dir = 0;
    let mut map: HashMap<(i64, i64), char> = HashMap::new();
    let mut coords: (i64, i64) = (0,0);
    let mut standing_on = 0;
    if starting == BLOCK_WHITE {
        map.insert(coords, BLOCK_WHITE);
    }

    let mut top_left: (i64, i64) = (0,0);
    let mut bottom_right: (i64, i64) = (0,0);

    loop {
        if *map.get(&coords).unwrap_or(&BLOCK_BLACK) != BLOCK_BLACK {
            standing_on = 1;
        }

        mac.set_input(&vec![standing_on]);
        if top_left.0 > coords.0 {
            top_left.0 = coords.0;
        }

        if top_left.1 < coords.1 {
            top_left.1 = coords.1;
        }

        if bottom_right.0 < coords.0 {
            bottom_right.0 = coords.0;
        }

        if bottom_right.1 > coords.1 {
            bottom_right.1 = coords.1;
        }


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
    (map, top_left, bottom_right)
}

pub fn part1() {

    let mut mac = Machine::new(&input::get_code(), &vec![]);
    let (painting, _, _) = paint(&mut mac, BLOCK_BLACK);


    println!("Painted {} tiles", painting.len()); // 2428
}

pub fn part2() {

    let mut mac = Machine::new(&input::get_code(), &vec![]);
    let (painting, tl, br) = paint(&mut mac, BLOCK_WHITE);
    let x0 = min(br.0, tl.0);
    let xx_max = max(br.0, tl.0);
    let mut yy = max(br.1, tl.1);
    let yy_max = min(br.1, tl.1);

    while yy >= yy_max {
        let mut xx = x0;
        while xx <= xx_max {
            print!("{}", painting.get(&(xx, yy)).unwrap_or(&BLOCK_BLACK));
            xx += 1;
        }
        print!("\n");
        yy -= 1;
    }


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
}
