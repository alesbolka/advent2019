use std::cmp::PartialEq;
use std::cmp::{min, max};

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

    pub fn get_grav(&mut self, other: &Self) -> (i64, i64, i64) {
        let dx = {
            if self.x > other.x {
                -1
            } else if self.x < other.x {
                1
            } else {
                0
            }
        };

        let dy = {
            if self.y > other.y {
                -1
            } else if self.y < other.y {
                1
            } else {
                0
            }
        };

        let dz = {
            if self.z > other.z {
                -1
            } else if self.z < other.z {
                1
            } else {
                0
            }
        };

        (dx, dy, dz)
    }

    pub fn step(&mut self, velocity: (i64, i64, i64)) {
        self.x += velocity.0;
        self.y += velocity.1;
        self.z += velocity.2;
    }

    pub fn get_coords(&self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }
}

impl PartialEq for Moon {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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
        assert_eq!((1,-1,0), m1.get_grav(&m2));
    }

    #[test]
    fn step_test() {
        let mut m1 = Moon::new("a", 3, 4, 2);
        m1.step((2, -5, 0));
        assert_eq!((5,-1,2), (m1.x, m1.y, m1.z));
    }
}