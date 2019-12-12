use std::cmp::PartialEq;
use std::cmp::{min, max};
use regex::{ Regex, Captures };

#[derive(Debug)]
pub struct Moon {
    x: i64,
    y: i64,
    z: i64,
    velocity: (i64, i64, i64),
    name: String,
}

impl Moon {
    pub fn new (name: &str, x: i64, y: i64, z: i64) -> Moon {
        Moon{
            x,
            y,
            z,
            velocity: (0,0,0),
            name: String::from(name),
        }
    }

    pub fn parse(raw: &str, designation: &str) -> Moon {
        lazy_static! {
            static ref MOON_REGEX: Regex = Regex::new(r"^<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>$").unwrap();
        }
        if !MOON_REGEX.is_match(raw) {
            panic!("Failed to parse \"{}\"", raw);
        }

        let caps = MOON_REGEX.captures(raw).unwrap();
        Moon::new(
            designation,
            caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
        )
    }

    pub fn apply_grav(&mut self, other: &Self) {
        self.velocity.0 += {
            if self.x > other.x {
                -1
            } else if self.x < other.x {
                1
            } else {
                0
            }
        };
        self.velocity.1 += {
            if self.y > other.y {
                -1
            } else if self.y < other.y {
                1
            } else {
                0
            }
        };
        self.velocity.2 += {
            if self.z > other.z {
                -1
            } else if self.z < other.z {
                1
            } else {
                0
            }
        };
    }

    pub fn step(&mut self) {
        self.x += self.velocity.0;
        self.y += self.velocity.1;
        self.z += self.velocity.2;
    }

    pub fn get_coords(&self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }

    pub fn get_velocity(&self) -> (i64, i64, i64) {
        self.velocity
    }
}

impl PartialEq for Moon {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
            self.x == other.x &&
            self.y == other.y &&
            self.z == other.z &&
            self.velocity == other.velocity
    }
}

impl Eq for Moon {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn velocity_test() {
        let mut m1 = Moon::new("a", 3, 4, 2);
        let m2 = Moon::new("b", 5, 1, 2);
        m1.apply_grav(&m2);
        assert_eq!((-1,1,0), m1.velocity);
    }

    #[test]
    fn step_test() {
        let mut m1 = Moon::new("a", 3, 4, 2);
        m1.velocity = (2, -5, 0);
        m1.step();
        assert_eq!((5,-1,2), (m1.x, m1.y, m1.z));
    }

    #[test]
    fn parse_test() {
        assert_eq!(Moon::new(&"x", -1, 0, 2), Moon::parse(&"<x=-1, y=0, z=2>", &"x"));
        assert_eq!(Moon::new(&"t", 2, -10, -7), Moon::parse(&"<x=2, y=-10, z=-7>", &"t"));
    }
}