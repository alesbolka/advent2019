use crate::task_03::point::{do_lines_intersect, find_x, Point};
use crate::task_03::Plane;

pub struct Wire {
    current_point: Point,
    lines: Vec<(Point, Point)>,
}

fn parse_move(raw: &str) -> (Plane, i32) {
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

impl Wire {
    pub fn new(origin: &Point) -> Wire {
        Wire {
            current_point: origin.clone(),
            lines: vec![],
        }
    }

    pub fn parse(raw: &str, origin: &Point) -> Wire {
        let mut wire = Wire::new(origin);
        for instruction in raw.split(',') {
            wire.travel(parse_move(instruction));
        }

        wire
    }

    pub fn travel_distance(&self, dest: &Point) -> i32 {
        let mut total: i32 = 0;

        for line in &self.lines {
            if dest.is_on_segment(&line) {
                total += line.0.dist(dest);
                return total
            }
            total += line.0.dist(&line.1);
        }
        -1 // dest is not on the wire
    }

    fn travel(&mut self, vector: (Plane, i32)) {
        let old_point = self.current_point.clone();
        self.current_point = self.current_point.apply_vector(vector);
        self.lines.push((old_point, self.current_point.clone()));
    }

    pub fn find_intersects(a: &Wire, b: &Wire) -> Vec<Point> {
        let mut res: Vec<Point> = vec![];

        for line1 in a.lines.iter() {
            for line2 in b.lines.iter() {
                if do_lines_intersect(line1, line2) {
                    res.push(find_x(line1, line2));
                }
            }
        }

        res
    }

    pub fn find_shortest_dist_by_path(origin: &Point, w1: &Wire, w2: &Wire) -> i32 {
        let crossings = Wire::find_intersects(&w1, &w2);

        match crossings.iter().map(|cr| {
            let d1 = w1.travel_distance(&cr);
            let d2 = w2.travel_distance(&cr);
            assert!(d1 > 0 && d2 > 0, "Invalid distance");
            d1 + d2
        }).min() {
            Some(x) => x,
            None => -1,
        }
    }
}
