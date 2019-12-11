#[derive(Debug)]
pub struct Asteroid {
    pub x: i32,
    pub y: i32,
}

impl Asteroid {
    pub fn new (x: i32, y: i32) -> Asteroid{
        Asteroid{
            x,
            y,
        }
    }

    pub fn vector (&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dy as f64).atan2(dx as f64)
    }
}

impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Asteroid {}