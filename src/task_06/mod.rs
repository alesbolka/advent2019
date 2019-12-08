mod input;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Body<'a> {
    name: &'a str,
    children: Vec<&'a str>,
    parent: Option<&'a str>,
}

impl<'a> Body<'a> {
    fn new(name: &'a str) -> Body {
        Body{
            name,
            children: vec![],
            parent: None,
        }
    }
}

pub fn parse_map<'a>(input: &'a str) -> HashMap<&'a str, Body<'a>> {
    let mut map: HashMap<&'a str, Body<'a>> = HashMap::new();
    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split(")").collect();
        if parts.len() != 2 {
            panic!("Invalid map spec {}", line);
        }

        // storing to var causes issues?...
        match map.entry(parts[0]) {
            Entry::Occupied(existing) => existing.into_mut().children.push(parts[1]),
            Entry::Vacant(pos) => {
                let mut new_body = Body::new(parts[0]);
                new_body.children.push(parts[1]);
                pos.insert(new_body);
            },
        };

        match map.entry(parts[1]) {
            Entry::Occupied(existing) => {
                existing.into_mut().parent = Some(parts[0]);
            },
            Entry::Vacant(pos) => {
                let mut new_body = Body::new(parts[1]);
                new_body.parent = Some(parts[0]);
                pos.insert(new_body);
            },
        };
    }

    map
}

fn count_orbits<'a>(map: &HashMap<&'a str, Body<'a>>, around: &'a str, offset: i32) -> i32 {
    let root = map.get(around).unwrap();
    let mut total = 0;
    for child in root.children.iter() {
        total += offset + 1;
        total += count_orbits(map, child, offset + 1);
    }
    total
}

fn min_transfers<'a>(map: &HashMap<&'a str, Body<'a>>, from: &'a str, to: &'a str) -> i32 {
    let mut chain1: Vec<&'a str> = vec![];
    let mut chain2: Vec<&'a str> = vec![];
    let mut p1 = match map.get(from) {
        Some(x) => x,
        None => panic!("No {} in map", from),
    };
    let mut p2 = match map.get(to) {
        Some(x) => x,
        None => panic!("No {} in map", from),
    };


    loop {
        match p1.parent {
            Some(name) => {
                chain1.push(name);
                p1 = map.get(name).unwrap();
            },
            None => break,
        };
    };

    loop {
        match p2.parent {
            Some(name) => {
                chain2.push(name);
                p2 = map.get(name).unwrap();
            },
            None => break,
        };
    };

    for (ii, name1) in chain1.iter().enumerate() {
        for (jj, name2) in chain2.iter().enumerate() {
            if name1 == name2 {
                return (ii + jj) as i32;
            }
        }
    }

    -1
}

pub fn part1() {
    let map = parse_map(input::PUZZLE);
    println!("{} orbits", count_orbits(&map, "COM", 0)); // 122782
}

pub fn part2() {
    let map = parse_map(input::PUZZLE);
    println!("Minimum transfers: {}", min_transfers(&map, "YOU", "SAN"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_constructor_and_orbit_count() {
        let map = parse_map(input::EXAMPLE_PART1);
        assert_eq!(42, count_orbits(&map, "COM", 0));
    }

    #[test]
    pub fn test_transfers_0() {
        let map = parse_map("A)B\nA)C");
        assert_eq!(0, min_transfers(&map, "B", "C"));
    }

    #[test]
    pub fn test_transfers_1() {
        let map = parse_map("A)B\nB)C");
        assert_eq!(1, min_transfers(&map, "B", "C"));
    }

    #[test]
    pub fn test_transfers_2() {
        let map = parse_map("A)B\nA)D\nD)C");
        assert_eq!(1, min_transfers(&map, "B", "C"));
    }

    #[test]
    pub fn test_transfers_3() {
        let map = parse_map("A)B\nA)D\nD)C\nB)E");
        assert_eq!(2, min_transfers(&map, "E", "C"));
    }

    #[test]
    pub fn test_transfers_sample() {
        let map = parse_map(input::EXAMPLE_PART2);
        assert_eq!(4, min_transfers(&map, "YOU", "SAN"));
    }
}
