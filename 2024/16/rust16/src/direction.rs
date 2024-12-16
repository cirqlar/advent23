#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

pub trait AddDirection {
    type Output;

    fn add_direction(self, rhs: &Direction, grid_size: usize) -> Self::Output;
}

impl AddDirection for usize {
    type Output = usize;

    fn add_direction(self, rhs: &Direction, grid_size: usize) -> Self::Output {
        match rhs {
            Direction::Up => self - grid_size,
            Direction::Down => self + grid_size,
            Direction::Left => self - 1,
            Direction::Right => self + 1,
        }
    }
}
