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

pub fn test() {
    let x = parse_map(input::EXAMPLE_RAW);

    match x.get("B") {
        Some(y) => println!("B has {} children", y.children.len()),
        None => println!("B not found"),
    };


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
