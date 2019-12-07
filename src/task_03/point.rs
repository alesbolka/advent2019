use crate::task_03::Plane;
use std::cmp;

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn apply_vector(&self, vector: (Plane, i32)) -> Point {
        let mut point = Point { ..*self };

        match vector.0 {
            Plane::X => point.x += vector.1,
            Plane::Y => point.y += vector.1,
        }

        point
    }

    pub fn shortest_distance(origin: &Point, list: &Vec<Point>) -> i32 {
        match list.iter().map(|a| a.dist(origin)).min() {
            Some(x) => x,
            None => -1,
        }
    }

    pub fn dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn is_on_segment(&self, line: &(Point, Point)) -> bool {
        let cross = (self.y - line.0.y) * (line.1.x - line.0.x) - (self.x - line.0.x) * (line.1.y - line.0.y);
        if cross.abs() > 0 {
            return false
        }

        let dot = (self.x - line.0.x) * (line.1.x - line.0.x) + (self.y - line.0.y)*(line.1.y - line.0.y);
        if dot < 0 {
            return false
        }
        let square = (line.1.x - line.0.x)*(line.1.x - line.0.x) + (line.1.y - line.0.y)*(line.1.y - line.0.y);

        dot <= square
    }

    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

pub fn orientation(a: &Point, b: &Point, c: &Point) -> u8 {
    let arg = (b.y - a.y) * (c.x - b.x) - (c.y - b.y) * (b.x - a.x);
    if arg < 0 {
        2 // counterclockwise
    } else if arg > 0 {
        1 // clockwise
    } else {
        0 // parralel
    }
}

pub fn overlap(a: &Point, b: &Point, c: &Point) -> bool {
    b.x <= cmp::max(a.x, c.x)
        && b.x >= cmp::min(a.x, c.x)
        && b.y <= cmp::max(a.y, c.y)
        && b.y >= cmp::min(a.y, c.y)
}

pub fn find_x(a: &(Point, Point), b: &(Point, Point)) -> Point {
    // we only do vertical and horizontal lines, so shortcut
    if a.0.x == a.1.x {
        Point { x: a.0.x, y: b.0.y }
    } else {
        Point { x: b.0.x, y: a.0.y }
    }
}

pub fn do_lines_intersect(a: &(Point, Point), b: &(Point, Point)) -> bool {
    if a.0.is_origin() && b.0.is_origin() {
        return false;
    }
    let or1 = orientation(&a.0, &a.1, &b.0);
    let or2 = orientation(&a.0, &a.1, &b.1);
    let or3 = orientation(&b.0, &b.1, &a.0);
    let or4 = orientation(&b.0, &b.1, &a.1);

    if or1 != or2 && or3 != or4 {
        return true;
    }

    // At least one combination is not colinear, not overlapping
    if or1 != 0 || or2 != 0 || or3 != 0 || or4 != 0 {
        return false;
    }

    if overlap(&a.0, &a.1, &b.0)
        || overlap(&a.0, &a.1, &b.1)
        || overlap(&b.0, &b.1, &a.0)
        || overlap(&b.0, &b.1, &a.1)
    {
        panic!("Overlap!");
    }

    false
}
