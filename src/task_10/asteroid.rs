#[derive(Debug)]
pub struct Asteroid {
    pub x: i32,
    pub y: i32,
    pub v: (i32, f64),
}

impl Asteroid {
    pub fn new (x: i32, y: i32) -> Asteroid{
        Asteroid{
            x,
            y,
            v: (0, 0.0),
        }
    }

    pub fn vector (&self, other: &Self) -> (i32, f64) {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (
            dx * dx + dy * dy,
            (dy as f64).atan2(dx as f64),
        )
    }
}

impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Asteroid {}