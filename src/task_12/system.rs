use super::moon::Moon;

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
    fn example_1_test_ales () {
        let mut system = System::new();
        system.moons.push(Moon::new("a", -1, 0, 2));
        system.moons.push(Moon::new("d", 2, -10, -7));
        system.moons.push(Moon::new("c", 4, -8, 8));
        system.moons.push(Moon::new("d", 3, 5, -1));

        system.step();

        assert_eq!((2, -1, 1), system.moons[0].get_coords());
    }
}