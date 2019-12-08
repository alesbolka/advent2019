mod input;
use std::collections::HashMap;
use std::ptr;

pub struct Map<'a> {
    bodies: HashMap<&'a str, Body<'a>>,
}

impl<'a> Map<'a> {
    pub fn new() -> Map<'a> {
        Map {
            bodies: HashMap::new(),
        }
    }

    fn add_orbit(&mut self, a: &'a str, b: &'a str) {
        // let mut parent = self.get_or_add_planet(a);
        // let mut child = self.get_or_add_planet(b);
        // parent.add_orbiter(child);
    }

    fn get_or_add_planet(&mut self, name: &'a str) -> &'a mut Body<'a> {
        self.bodies.entry(name).or_insert_with(|| Body::new(name))
    }
}

pub struct Body<'a> {
    name: &'a str,
    children: Vec<&'a Body<'a>>,
    parent: Option<&'a Body<'a>>,
}

impl<'a> Body<'a> {
    fn new(name: &'a str) -> Body {
        Body{
            name,
            children: vec![],
            parent: None,
        }
    }

    fn add_orbiter(&mut self, child: &'a Body) {
        self.children.push(child);
    }
}

pub fn parse_map(input: &str) -> Map {
    let mut map = Map::new();
    for line in input.lines() {
        if !line.contains(")") {
            panic!("No ) in {}", line);
        }
        let parts: Vec<&str> = line.split(")").collect();
        if parts.len() != 2 {
            panic!("Invalid map spec {}", line);
        }
        let planet1 = map.get_or_add_planet(parts[0]);
        // map.add_orbit(parts[0], parts[1]);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let map = parse_map(input::EXAMPLE_RAW);
        assert_eq!(42, 42);
    }
}
