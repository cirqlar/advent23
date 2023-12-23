use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, fmt::Display, ops::Add};

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn new(x: isize, y: isize) -> Vec2 {
        Vec2 { x, y }
    }

    fn valid(&self, x_max: isize, y_max: isize) -> bool {
        (0..x_max).contains(&self.x) && (0..y_max).contains(&self.y)
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

impl From<(isize, isize)> for Vec2 {
    fn from(value: (isize, isize)) -> Self {
        Vec2::new(value.0, value.1)
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Vec2 { x, y } if *x == 0 && *y == -1 => "^",
                Vec2 { x, y } if *x == 0 && *y == 1 => "v",
                Vec2 { x, y } if *x == -1 && *y == 0 => "<",
                Vec2 { x, y } if *x == 1 && *y == 0 => ">",
                _ => unreachable!(),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    pos: Vec2,
    dir: Option<Vec2>,
    count: usize,
}

fn process(input: &str) -> usize {
    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Vec2::new(x as isize, y as isize),
                    c.to_digit(10).expect("can") as usize,
                )
            })
        })
        .collect::<HashMap<_, _>>();
    let start_pos = Vec2::new(0, 0);
    let last_pos = Vec2::new(width - 1, height - 1);
    let start_node = Node {
        pos: start_pos,
        dir: None,
        count: 0,
    };

    let result = dijkstra(
        &start_node,
        |n| {
            let mut new_dirs = vec![
                Vec2::new(1, 0),
                Vec2::new(0, -1),
                Vec2::new(-1, 0),
                Vec2::new(0, 1),
            ];

            if n.dir.is_some() {
                let dir = n.dir.unwrap();
                if dir.x == -1 && dir.y == 0 {
                    new_dirs.remove(0);
                } else if dir.x == 1 && dir.y == 0 {
                    new_dirs.remove(2);
                } else if dir.x == 0 && dir.y == 1 {
                    new_dirs.remove(1);
                } else if dir.x == 0 && dir.y == -1 {
                    new_dirs.remove(3);
                }
            }

            new_dirs
                .into_iter()
                .filter_map(|dir| {
                    let next_pos = n.pos + dir;
                    if !next_pos.valid(width, height) {
                        return None;
                    }

                    if n.count >= 3 && dir == n.dir.unwrap() {
                        return None;
                    }

                    let count = if n.dir.is_some() && dir == n.dir.unwrap() {
                        n.count + 1
                    } else {
                        1
                    };
                    let cost = *grid.get(&next_pos).unwrap();
                    Some((
                        Node {
                            pos: next_pos,
                            dir: Some(dir),
                            count,
                        },
                        cost,
                    ))
                })
                .collect::<Vec<_>>()
        },
        |n| n.pos == last_pos,
    );

    // for y in 0..height {
    //     for x in 0..width {
    //         if let Some(st) = result
    //             .as_ref()
    //             .unwrap()
    //             .0
    //             .iter()
    //             .find(|p| p.pos == Vec2::new(x, y))
    //         {
    //             print!("{}", st.dir.unwrap_or(Vec2::new(1, 0)));
    //         } else {
    //             print!("{}", grid.get(&Vec2::new(x, y)).expect("has"));
    //         }
    //     }
    //     println!();
    // }

    result.expect("Shortest Path").1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d17_part_1_is_correct() {
        let input = include_str!("../../../input/test.txt");

        assert_eq!(process(input), 102);
    }
}
