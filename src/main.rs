use std::cmp;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn apply_vector(&self, vector: (Plane, i32)) -> Point {
        let mut point = Point { ..*self };

        match vector.0 {
            Plane::X => point.x += vector.1,
            Plane::Y => point.y += vector.1,
        }

        point
    }

    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point{ x: self.x, y: self.y }
    }
}

#[derive(Copy, Clone)]
enum Plane {
    X,
    Y,
}

struct Wire {
    current_point: Point,
    lines: Vec<(Point, Point)>,
}

fn orientation(a: &Point, b: &Point, c: &Point) -> u8 {
    let arg = (b.y - a.y)*(c.x - b.x) - (c.y - b.y)*(b.x - a.x);
    if arg < 0 {
        2 // counterclockwise
    } else if arg > 0 {
        1 // clockwise
    } else {
        0 // parralel
    }
}

fn overlap(a: &Point, b: &Point, c: &Point) -> bool {
    b.x <= cmp::max(a.x, c.x) &&
        b.x >= cmp::min(a.x, c.x) &&
        b.y <= cmp::max(a.y, c.y) &&
        b.y >= cmp::min(a.y, c.y)
}

fn new_wire (origin: Point) -> Wire {
    Wire{ current_point: origin.clone(), lines: vec!{} }
}

impl Wire {
    fn path(&mut self, directions: Vec<String>) {
        for raw in directions {
            self.travel(parse_move(raw));
        }
    }

    fn travel(&mut self, vector: (Plane, i32)) {
        let old_point = self.current_point.clone();
        self.current_point = self.current_point.apply_vector(vector);
        self.lines.push((old_point, self.current_point.clone()));
    }

    fn find_intersects(&self, other: &Wire) -> Vec<Point> {
        let mut res: Vec<Point> = vec!{};

        for line1 in self.lines.iter() {
            for line2 in other.lines.iter() {
                if line_intersect(line1, line2) {
                    println!("{:?}, {:?}", line1, line2);
                }
            }
        }

        res
    }
}

fn line_intersect (a: &(Point, Point), b: &(Point, Point)) -> bool {
    if a.0.is_origin() && b.0.is_origin() {
        return false;
    }
    let or1 = orientation(&a.0, &a.1, &b.0);
    let or2 = orientation(&a.0, &a.1, &b.1);
    let or3 = orientation(&b.0, &b.1, &a.0);
    let or4 = orientation(&b.0, &b.1, &a.1);

    if or1 != or2 && or3 != or4 {
        return true
    }

    // At least one combination is not colinear, not overlapping
    if or1 != 0 || or2 != 0 || or3 != 0 || or4 != 0 {
        return false
    }

    if
        overlap(&a.0, &a.1, &b.0) ||
        overlap(&a.0, &a.1, &b.1) ||
        overlap(&b.0, &b.1, &a.0) ||
        overlap(&b.0, &b.1, &a.1)
    {
        panic!("Overlap!");
    }

    false
}

fn parse_move(raw: String) -> (Plane, i32) {
    let parts = raw.split_at(1);
    let dist = match parts.1.parse::<i32>() {
        Ok(x) => x,
        Err(e) => panic!(e),
    };
    match parts.0 {
        "L" => (Plane::X, -dist),
        "R" => (Plane::X, dist),
        "U" => (Plane::Y, dist),
        "D" => (Plane::Y, -dist),
        _ => panic!("Invalid command {}", raw),
    }
}

fn main () {
    let input1: Vec<String> = vec![String::from("R8"),String::from("U5"),String::from("L5"),String::from("D3")];
    let input2: Vec<String> = vec![String::from("U7"),String::from("R6"),String::from("D4"),String::from("L4")];
    // let input1: Vec<String> = vec![String::from("R75"),String::from("D30"),String::from("R83"),String::from("U83"),String::from("L12"),String::from("D49"),String::from("R71"),String::from("U7"),String::from("L72")];
    // let input2: Vec<String> = vec![String::from("U62"),String::from("R66"),String::from("U55"),String::from("R34"),String::from("D71"),String::from("R55"),String::from("D58"),String::from("R83")];
    let origin = Point { x: 0, y: 0 };

    let mut wire1 = new_wire(origin.clone());
    wire1.path(input1);
    let mut wire2 = new_wire(origin.clone());
    wire2.path(input2);

    let points = wire1.find_intersects(&wire2);
    println!("{:?}", points);
}