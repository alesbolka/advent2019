use super::asteroid::Asteroid;

pub struct Map {
    asteroids: Vec<Asteroid>,
    best: usize,
}

impl Map {
    pub fn parse (raw: &str) -> Map {
        let mut map = Map{
            asteroids: vec![],
            best: 0,
        };
        let mut xx = 0;
        let mut yy = 0;

        for bb in raw.bytes() {
            if bb == b'#' {
                map.asteroids.push(Asteroid::new(xx, yy))
            }
            xx += 1;
            if bb == b'\n' {
                yy += 1;
                xx = 0
            }
        }

        map
    }

    pub fn find_best (&self) -> (Option<usize>, i32) {
        let mut max = 0;
        let mut max_ii = 0;
        for (ii, roid) in self.asteroids.iter().enumerate() {
            let mut visible: Vec<f64> = vec![];
            for other in &self.asteroids {
                if roid == other {
                    continue;
                }
                let vv = roid.vector(other);

                if !visible.contains(&vv.1) {
                    visible.push(vv.1);
                }
            }
            if max < visible.len() {
                max = visible.len();
                max_ii = ii;
            }
        }

        if max == 0 {
            return (
                None,
                0
            )
        }

        (Some(max_ii), max as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::input;

    #[test]
    fn simple_parse() {
        // #.#
        // ...
        // #.#
        let map = Map::parse("#.#\n...\n#.#");
        assert_eq!(4, map.asteroids.len());
        assert_eq!(Asteroid::new(0,0), map.asteroids[0]);
        assert_eq!(Asteroid::new(2,0), map.asteroids[1]);
        assert_eq!(Asteroid::new(0,2), map.asteroids[2]);
        assert_eq!(Asteroid::new(2,2), map.asteroids[3]);
    }

    #[test]
    fn simple_1() {
        // #.#
        // ...
        // #.#
        let map = Map::parse("#.#\n...\n#.#");
        let res = map.find_best();
        assert_eq!(0 as usize, res.0.unwrap());
        assert_eq!(3, res.1);
    }


    #[test]
    fn simple_2() {
        // #.#.#
        // ..#..
        // #.#.#
        let map = Map::parse("#.#.#\n..#..\n#.#.#");
        let res = map.find_best();
        assert_eq!(3 as usize, res.0.unwrap());
        assert_eq!(6, res.1);
    }

    #[test]
    fn simple_3() {
        // #####
        // ..#..
        // #.#.#
        let map = Map::parse("#####\n..#..\n#.#.#");
        let res = map.find_best();
        assert_eq!(5 as usize, res.0.unwrap());
        assert_eq!(8, res.1);
    }


    #[test]
    fn demo_1() {
        // .7..7
        // .....
        // 67775
        // ....7
        // ...87
        let map = Map::parse(input::DEMO1);
        let res = map.find_best();
        assert_eq!(8 as usize, res.0.unwrap());
        assert_eq!(8, res.1);
    }

    #[test]
    fn demo_2() {
        let map = Map::parse(input::DEMO2);
        let res = map.find_best();
        assert_eq!(33, res.1);
    }

    #[test]
    fn demo_3() {
        let map = Map::parse(input::DEMO3);
        let res = map.find_best();
        assert_eq!(35, res.1);
    }

    #[test]
    fn demo_4() {
        let map = Map::parse(input::DEMO4);
        let res = map.find_best();
        assert_eq!(41, res.1);
    }

    #[test]
    fn demo_5() {
        let map = Map::parse(input::DEMO5);
        let res = map.find_best();
        assert_eq!(210, res.1);
    }
}