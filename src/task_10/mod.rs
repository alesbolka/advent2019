mod input;
mod map;
mod asteroid;
use map::Map;

pub fn part1() {
    let map = Map::parse(input::PART1);
    let res = map.find_best();
    // 288 asteroids seen by asteroid 258
    println!("I see: {} asteroids from {}", res.1, res.0.unwrap());
    assert_eq!(258, res.0.unwrap());
}

pub fn part2() {
    let map = Map::parse(input::PART1);
    let winner = map.radial_shooting(258, 200).unwrap();
    println!("200th asteroid is at: ({}, {}), also IDed as {}", winner.x, winner.y, winner.x * 100 + winner.y);
    // let l
}
