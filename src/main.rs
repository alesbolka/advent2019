struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn apply_vector(&self, vector: (Direction, i32)) -> Point {
        let mut point = Point { ..*self };

        match vector.0 {
            Direction::Left => point.x -= vector.1,
            Direction::Right => point.x += vector.1,
            Direction::Up => point.y += vector.1,
            Direction::Down => point.y -= vector.1,
        }

        point
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point{ x: self.x, y: self.y }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Wire {
    currentPoint: Point,
    lines: Vec<(Point, Direction, i32)>,
}

fn new_wire (origin: Point) -> Wire {
    Wire{ currentPoint: origin.clone(), lines: vec!{} }
}

impl Wire {
    fn path(&mut self, directions: Vec<String>) {
        for raw in directions {
            self.travel(parse_move(raw));
        }
    }

    fn travel(&mut self, vector: (Direction, i32)) {
        self.lines.push((self.currentPoint.clone(), vector.0, vector.1));
        self.currentPoint = self.currentPoint.apply_vector(vector);
    }
}

fn parse_move(raw: String) -> (Direction, i32) {
    let parts = raw.split_at(1);
    let dir = match parts.0 {
        "L" => Direction::Left,
        "R" => Direction::Right,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => panic!("Invalid command {}", raw),
    };

    let dist = match parts.1.parse::<i32>() {
        Ok(x) => x,
        Err(e) => panic!(e),
    };
    (dir, dist)
}

fn main () {
    let input1: Vec<String> = vec![String::from("R75"),String::from("D30"),String::from("R83"),String::from("U83"),String::from("L12"),String::from("D49"),String::from("R71"),String::from("U7"),String::from("L72")];
    let input2: Vec<String> = vec![String::from("U62"),String::from("R66"),String::from("U55"),String::from("R34"),String::from("D71"),String::from("R55"),String::from("D58"),String::from("R83")];
    let origin = Point { x: 0, y: 0 };

    let mut wire1 = new_wire(origin.clone());
    wire1.path(input1);
}