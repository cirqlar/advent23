use std::{
    collections::{HashMap, VecDeque},
    ops::Add,
};

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<Direction> for Vec2<isize> {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Vec2(0, -1),
            Direction::Down => Vec2(0, 1),
            Direction::Right => Vec2(1, 0),
            Direction::Left => Vec2(-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2<T>(T, T);
// type Vec2<T> = (T, T);

impl<T: Add> Add for Vec2<T> {
    type Output = Vec2<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<'a, T> Add for &'a Vec2<T>
where
    &'a T: Add,
{
    type Output = Vec2<<&'a T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(&self.0 + &rhs.0, &self.1 + &rhs.1)
    }
}

impl TryFrom<Vec2<usize>> for Vec2<isize> {
    type Error = <isize as TryFrom<usize>>::Error;

    fn try_from(value: Vec2<usize>) -> Result<Self, Self::Error> {
        Ok(Vec2(value.0.try_into()?, value.1.try_into()?))
    }
}
impl TryFrom<Vec2<isize>> for Vec2<usize> {
    type Error = <usize as TryFrom<isize>>::Error;

    fn try_from(value: Vec2<isize>) -> Result<Self, Self::Error> {
        Ok(Vec2(value.0.try_into()?, value.1.try_into()?))
    }
}

fn process(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut light_path: HashMap<Vec2<usize>, Vec<Direction>> = HashMap::new();
    let mut check_next = VecDeque::new();
    check_next.push_back((Vec2(0, 0), Direction::Right));

    while let Some((v, dir)) = check_next.pop_front() {
        let y = v.1;
        let x = v.0;
        let mut next: [Option<Direction>; 2] = [None; 2];
        match grid[y][x] {
            '.' => {
                next[0] = Some(dir);
            }
            '\\' => {
                next[0] = match dir {
                    Direction::Up => Some(Direction::Left),
                    Direction::Down => Some(Direction::Right),
                    Direction::Right => Some(Direction::Down),
                    Direction::Left => Some(Direction::Up),
                };
            }
            '/' => {
                next[0] = match dir {
                    Direction::Up => Some(Direction::Right),
                    Direction::Down => Some(Direction::Left),
                    Direction::Right => Some(Direction::Up),
                    Direction::Left => Some(Direction::Down),
                };
            }
            '|' => {
                if dir == Direction::Left || dir == Direction::Right {
                    next[0] = Some(Direction::Up);
                    next[1] = Some(Direction::Down);
                } else {
                    next[0] = Some(dir);
                }
            }
            '-' => {
                if dir == Direction::Up || dir == Direction::Down {
                    next[0] = Some(Direction::Right);
                    next[1] = Some(Direction::Left);
                } else {
                    next[0] = Some(dir);
                }
            }
            _ => unreachable!(),
        }

        let mut next = next.into_iter();
        while let Some(Some(new_dir)) = next.next() {
            if let Some(us_v) = light_path.get(&v) {
                if us_v.contains(&new_dir) {
                    continue;
                }
            }

            light_path
                .entry(v)
                .and_modify(|us_v| us_v.push(new_dir))
                .or_insert(vec![new_dir]);

            let v: Vec2<isize> = v.try_into().expect("can");
            let next_loc = v + new_dir.into();

            if next_loc.0 >= 0
                && next_loc.0 < (width as isize)
                && next_loc.1 >= 0
                && next_loc.1 < (height as isize)
            {
                check_next.push_back((next_loc.try_into().expect("can"), new_dir));
            }
        }
    }

    light_path.keys().count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d16_part_1_is_correct() {
        let input = include_str!("../../../input/test.txt");

        assert_eq!(process(input), 46);
    }
}
