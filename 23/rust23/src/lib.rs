use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vec2 {
    pub y: isize,
    pub x: isize,
}

impl Vec2 {
    pub fn new(x: isize, y: isize) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn valid(&self, bounds: &Vec2) -> bool {
        (0..bounds.x).contains(&self.x) && (0..bounds.y).contains(&self.y)
    }

    pub fn map_to(&self, x_max: isize, y_max: isize) -> Vec2 {
        Vec2 {
            x: self.x.rem_euclid(x_max),
            y: self.y.rem_euclid(y_max),
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a> Add for &'a Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(isize, isize)> for Vec2 {
    fn from(value: (isize, isize)) -> Self {
        Vec2::new(value.0, value.1)
    }
}
