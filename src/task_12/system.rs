use super::moon::Moon;
use super::input;

pub struct System {
    moons: Vec<Moon>,
}

pub fn tuple_sum(a: (i64, i64, i64), b: (i64, i64, i64)) -> (i64, i64, i64){
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

impl System {
    pub fn new () -> System {
        System {
            moons: vec![],
        }
    }

    pub fn parse (raw: &str) -> System{
        let mut system = System::new();
        for (ii, line) in raw.trim().split('\n').enumerate() {
            system.moons.push(Moon::parse(line, &format!("{}", ii)));
        }

        system
    }

    pub fn step (&mut self) {
        for ii in 0..self.moons.len() {
            let (_, list) = self.moons.split_at_mut(ii);
            let (moon, others) = list.split_first_mut().unwrap();
            for other in others {
                moon.apply_grav(&other);
                other.apply_grav(moon);
            }

            moon.step();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test () {
        let system = System::parse(input::EXAMPLE);
        let m1 = Moon::new("0", -1, 0, 2);
        let m2 = Moon::new("1", 2, -10, -7);
        let m3 = Moon::new("2", 4, -8, 8);
        let m4 = Moon::new("3", 3, 5, -1);

        assert_eq!(m1, system.moons[0]);
        assert_eq!(m2, system.moons[1]);
        assert_eq!(m3, system.moons[2]);
        assert_eq!(m4, system.moons[3]);
    }

    #[test]
    fn example_01_ales () {
        let mut system = System::parse(input::EXAMPLE);

        // Step 1
        system.step();
        assert_eq!((2, -1, 1), system.moons[0].get_coords());
        assert_eq!((3, -7, -4), system.moons[1].get_coords());
        assert_eq!((1, -7, 5), system.moons[2].get_coords());
        assert_eq!((2, 2, 0), system.moons[3].get_coords());
        assert_eq!((3, -1, -1), system.moons[0].get_velocity());
        assert_eq!((1, 3, 3), system.moons[1].get_velocity());
        assert_eq!((-3, 1, -3), system.moons[2].get_velocity());
        assert_eq!((-1, -3, 1), system.moons[3].get_velocity());

        // Step 2
        system.step();
        assert_eq!((5, -3, -1), system.moons[0].get_coords());
        assert_eq!((1, -2, 2), system.moons[1].get_coords());
        assert_eq!((1, -4, -1), system.moons[2].get_coords());
        assert_eq!((1, -4, 2), system.moons[3].get_coords());
        assert_eq!((3, -2, -2), system.moons[0].get_velocity());
        assert_eq!((-2, 5, 6), system.moons[1].get_velocity());
        assert_eq!((0, 3, -6), system.moons[2].get_velocity());
        assert_eq!((-1, -6, 2), system.moons[3].get_velocity());
    }
}