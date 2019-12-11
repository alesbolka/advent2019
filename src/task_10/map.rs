use super::asteroid::Asteroid;
use std::f64::consts::PI;
use std::collections::HashMap;

pub struct Map {
    asteroids: Vec<Asteroid>,
    best: usize,
    station: usize,
}

impl Map {
    pub fn parse (raw: &str) -> Map {
        let mut map = Map{
            asteroids: vec![],
            station: 0,
            best: 0,
        };
        let mut xx = 0;
        let mut yy = 0;

        for bb in raw.bytes() {
            if bb == b'#' {
                map.asteroids.push(Asteroid::new(xx, yy));
            }
            if bb == b'X' {
                map.station = map.asteroids.len();
                map.asteroids.push(Asteroid::new(xx, yy));
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

                if !visible.contains(&vv) {
                    visible.push(vv);
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

    pub fn radial_shooting (&self, base: usize, last: usize) -> Option<&Asteroid> {
        let station = &self.asteroids[base];
        let mut order: Vec<(i64)> = vec![];
        let mut horror: HashMap<i64, Vec<(i32, usize)>> = HashMap::new();

        for (ii, roid) in self.asteroids.iter().enumerate() {
            let dx = station.x - roid.x;
            let dy = station.y - roid.y;

            if dx == dy && dx == 0 {
                continue;
            }

            // Y axis is our 0, so offset by -PI/2
            let mut rad = (dy as f64).atan2(dx as f64) - PI / 2.0;
            if rad < 0.0 {
                rad += 2.0 * PI;
            }

            let rad = (rad * 100000000.0) as i64;// What are rounding errors?
            if !horror.contains_key(&rad) {
                horror.insert(rad, vec![]);
                order.push(rad);
            }
            horror.get_mut(&rad).unwrap().push((dx * dx + dy * dy, ii));
        };

        order.sort();

        let mut counter = 0;
        let first_loop = order.len();
        while counter < self.asteroids.len() {
            let ii = order[counter % first_loop];
            if counter < first_loop {
                // First iteration, sort array
                horror.get_mut(&ii).unwrap().sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            }

            if horror.get(&ii).unwrap().len() < 1 {
                println!("Skipping");
                continue;
            }
            counter += 1;
            let res = horror.get_mut(&ii).unwrap().remove(0);

            if counter >= last {
                return self.asteroids.get(res.1);
            }
        }
        None
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

    #[test]
    fn shoot_simple_1 () {
        // 0.1.2
        // ..3..
        // 4.5.6
        let map = Map::parse("#.#.#\n..#..\n#.#.#");
        let rr = map.radial_shooting(3, 1).unwrap();
        assert_eq!(2, rr.x);
        assert_eq!(0, rr.y);
        let rr = map.radial_shooting(3, 3).unwrap();
        assert_eq!(4, rr.x);
        assert_eq!(2, rr.y);
        let rr = map.radial_shooting(6, 2).unwrap();
        assert_eq!(2, rr.x);
        assert_eq!(2, rr.y);
    }


    #[test]
    fn shoot_example () {
        // .#....###24...#..
        // ##...##.13#67..9#
        // ##...#...5.8####.
        // ..#.....X...###..
        // ..#.#.....#....##
        let map = Map::parse(input::TARGET01);
        let rr = map.radial_shooting(map.station, 1).unwrap();
        assert_eq!(1, rr.y);
        assert_eq!(8, rr.x);
        let rr = map.radial_shooting(map.station, 2).unwrap();
        assert_eq!(0, rr.y);
        assert_eq!(9, rr.x);
        let rr = map.radial_shooting(map.station, 8).unwrap();
        assert_eq!(2, rr.y);
        assert_eq!(11, rr.x);
    }
}