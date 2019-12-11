mod input;
mod map;
mod asteroid;
use map::Map;

pub fn part1() {
    let map = Map::parse(input::PART1);
    let res = map.find_best();
    println!("I see: {} asteroids", res.1);
}
