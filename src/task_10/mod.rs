mod input;
use std::f32::{ INFINITY, NEG_INFINITY };
const EPS: f32 = 0.00001;

struct Map {
    asteroids: Vec<Asteroid>,
    lines: Vec<Line>,
}

impl Map {
    pub fn get_seen_asteroids(&self, index: usize) -> u32 {
        let mut count: u32 = 0; // Maximum
        for line in &self.lines {
            // println!("Line: {:?}\n {:?} => {}", line, line.roids, line.seen_roids(index));
            count += line.seen_roids(index);
        }

        count
    }

    pub fn get_highest_visible(&self) -> u32 {
        let mut max = 0;
        for ii in 0..self.asteroids.len() {
            let res = self.get_seen_asteroids(ii);
            if res > max {
                max = res;
            }
        }

        max
    }
}

struct Asteroid {
    x: f32,
    y: f32,
}

impl Asteroid {
    fn new (x: f32, y: f32) -> Asteroid {
        Asteroid{ x, y }
    }

    fn int (x: i32, y: i32) -> Asteroid {
        Self::new(x as f32, y as f32)
    }

}

#[derive(Debug)]
struct Line {
    k: f32,
    n: f32,
    min: f32,
    min_index: usize,
    max: f32,
    max_index: usize,
    roids: Vec<usize>,
}

impl Line {
    fn new (k: f32, n: f32) -> Line {
        Line{
            k,
            n,
            min: INFINITY,
            min_index: 0,
            max: 0.0,
            max_index: 0,
            roids: vec![],
        }
    }

    fn from_roids(a: &Asteroid, b: &Asteroid) -> Line {
        Line::math(
            a.x,
            a.y,
            b.x,
            b.y,
        )
    }

    fn math (x1: f32, y1:f32, x2:f32, y2:f32) -> Line {
        let mut k: f32 = (y2 - y1) / (x2 - x1);
        let mut n: f32 = (y2 - k * x2) as f32;
        if k == NEG_INFINITY || k == INFINITY || k == std::f32::NAN {
            // Vertical line
            k = INFINITY;
            n = x1;
        }

        Line::new(k, n)
    }

    fn add_point(&mut self, x: f32, y: f32, ii: usize) {
        self.roids.push(ii);
        if self.k == INFINITY {
            if self.max < y {
                self.max = y;
                self.max_index = ii;
            }
            if self.min > y {
                self.min = y;
                self.min_index = ii;
            }
            return;
        }
        if self.max < x {
            self.max = x;
            self.max_index = ii;
        }
        if self.min > x {
            self.min = x;
            self.min_index = ii;
        }
    }

    fn seen_roids(&self, index: usize) -> u32 {
        // println!("{}, {}, {}", index, self.max_index, self.min_index);
        if self.max_index == index || self.min_index == index {
            1 // This asteroid is on this line, but is at the end
        } else if self.roids.contains(&index) {
            2 // Is on the line, but not on the end - sees 2 roids
        } else {
            0 // Is not on the line
        }
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        // Vertical special comparison
        self.k == INFINITY &&
        other.k == INFINITY &&
        (self.n - other.n).abs() < EPS ||
        // Regular
        (self.k - other.k).abs() < EPS
        && (self.n - other.n).abs() < EPS
    }
}

impl Eq for Line {}


fn parse_map(raw: &str) -> Map {
    let mut xx = 0.0;
    let mut yy = 0.0;
    let mut asteroids: Vec<Asteroid> = vec![];
    let mut lines: Vec<Line> = vec![];
    let mut counter: usize = 0;

    for ch in raw.chars() {
        if ch == '#' {
            asteroids.push(Asteroid::new(xx, yy));
            'roidLoop: for (ii, roid) in asteroids.iter().enumerate() {
                if ii == counter {
                    continue;
                }

                let mut line = Line::from_roids(&asteroids[counter], roid);
                for sub in &mut lines {
                    if &line == sub {
                        sub.add_point(xx, yy, counter);
                        continue 'roidLoop;
                    }
                }
                line.add_point(roid.x, roid.y, ii);
                line.add_point(xx, yy, counter);

                lines.push(line);
            }
            counter += 1;
        } else if ch == '\n' {
            xx = 0.0;
            yy += 1.0;
            continue;
        }
        xx += 1.0;
    }

    Map{
        asteroids,
        lines,
    }
}


pub fn part1() {
    let map = parse_map(input::PART1);
    println!("I see: {} asteroids", map.get_highest_visible());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_cmp() {
        let l1 = Line::new(22.0 / 7.0, 4.0 / 3.0);
        let l2 = Line::new(33.0 / 10.5, 6.0 / 4.5);
        assert_eq!(l1, l2, "First");

        let l1 = Line::new(INFINITY, 2.0);
        let l2 = Line::new(INFINITY, 2.0);
        assert_eq!(l1, l2, "Vertical, same x");

        let l1 = Line::new(INFINITY, 3.0);
        let l2 = Line::new(INFINITY, 2.0);
        assert_ne!(l1, l2, "Vertical, different x");

        let l1 = Line::new(0.0, 3.0);
        let l2 = Line::new(0.0, 3.0);
        assert_eq!(l1, l2, "Horizontal, same y");

        let l1 = Line::new(0.0, 2.0);
        let l2 = Line::new(0.0, 3.0);
        assert_ne!(l1, l2, "Horizontal, different y");

        let l1 = Line::new(INFINITY, 2.0);
        let l2 = Line::new(2.0, 2.0);
        assert_ne!(l1, l2, "Vertical & random");
    }

    #[test]
    fn line_tests() {
        // .ad..b
        // ......
        // .c....
        // .....e
        let a = Asteroid::int(1, 0);
        let b = Asteroid::int(4, 0);
        let c = Asteroid::int(1, 3);
        let d = Asteroid::int(1, 1);
        let e = Asteroid::int(5, 4);
        let horizontal_0 = Line::new(0.0, 0.0);
        let vertical_1 = Line::new(INFINITY, 1.0);
        let diagonal = Line::new(0.75, 0.25);

        let l1 = Line::from_roids(&a, &b);
        assert_eq!(horizontal_0, l1, "first comparison");
        let l2 = Line::from_roids(&a, &c);
        assert_eq!(vertical_1, l2, "second comparison");
        let l3 = Line::from_roids(&e, &d);
        assert_eq!(diagonal, l3, "third comparison");
    }

    #[test]
    fn simple_1() {
        // #.#
        // ...
        // #.#
        let map = parse_map("#.#\n...\n#.#");
        assert_eq!(map.lines.len(), 6);
        assert_eq!(3, map.get_seen_asteroids(0));
        assert_eq!(3, map.get_seen_asteroids(1));
        assert_eq!(3, map.get_seen_asteroids(2));
        assert_eq!(3, map.get_seen_asteroids(3));
    }

    #[test]
    fn simple_2() {
        // #.#.#
        // ..#..
        // #.#.#
        let map = parse_map("#.#.#\n..#..\n#.#.#");
        assert_eq!(4, map.get_seen_asteroids(0));
        assert_eq!(6, map.get_seen_asteroids(3));
    }

    #[test]
    fn simple_3() {
        // #####
        // ..#..
        // #.#.#
        let map = parse_map("#####\n..#..\n#.#.#");
        assert_eq!(6, map.get_seen_asteroids(1));
        // assert_eq!(6, map.get_seen_asteroids(3));
    }

    #[test]
    fn demo_1() {
        // .7..7
        // .....
        // 67775
        // ....7
        // ...87
        let map = parse_map(input::DEMO1);
        for (ii, &expected) in vec![7,7,6,7,7,7,5,7,8,7].iter().enumerate() {
            assert_eq!(expected, map.get_seen_asteroids(ii), "Failed comparison {}", ii);
        }
        assert_eq!(8, map.get_highest_visible());
    }

    #[test]
    fn demo_2() {
        let map = parse_map(input::DEMO2);
        assert_eq!(33, map.get_highest_visible());
    }

    #[test]
    fn demo_3() {
        let map = parse_map(input::DEMO3);
        assert_eq!(35, map.get_highest_visible());
    }

    #[test]
    fn demo_4() {
        let map = parse_map(input::DEMO4);
        assert_eq!(41, map.get_highest_visible());
    }

    #[test]
    #[ignore] // to slow for continuous runs
    fn demo_5() {
        let map = parse_map(input::DEMO5);
        assert_eq!(210, map.get_highest_visible());
    }
}