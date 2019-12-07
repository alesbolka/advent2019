use crate::task_03::point::{ Point, find_x, do_lines_intersect };
use crate::task_03::Plane;

pub struct Wire {
  current_point: Point,
  lines: Vec<(Point, Point)>,
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

impl Wire {
  pub fn new(origin: Point) -> Wire {
    Wire{ current_point: origin.clone(), lines: vec!{} }
  }

  pub fn path(&mut self, directions: Vec<String>) {
      for raw in directions {
          self.travel(parse_move(raw));
      }
  }

  fn travel(&mut self, vector: (Plane, i32)) {
      let old_point = self.current_point.clone();
      self.current_point = self.current_point.apply_vector(vector);
      self.lines.push((old_point, self.current_point.clone()));
  }

  pub fn find_intersects(a: &Wire, b: &Wire) -> Vec<Point> {
      let mut res: Vec<Point> = vec!{};

      for line1 in a.lines.iter() {
          for line2 in b.lines.iter() {
              if do_lines_intersect(line1, line2) {
                  res.push(find_x(line1, line2));
              }
          }
      }

      res
  }
}