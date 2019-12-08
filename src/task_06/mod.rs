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

pub fn part1() {
    let map = parse_map(input::PUZZLE);
    println!("{} orbits", count_orbits(&map, "COM", 0)); // 122782
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_constructor_and_orbit_count() {
        let map = parse_map(input::EXAMPLE_RAW);
        assert_eq!(42, count_orbits(&map, "COM", 0));
    }
}
