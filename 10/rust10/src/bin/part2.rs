use std::{collections::HashMap, ops::Add};

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Vec2<T>(T, T);

impl<T: Add + Copy> Add for &Vec2<T> {
    type Output = Vec2<<T as std::ops::Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn process(input: &str) -> isize {
    let positions = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_position = positions
        .iter()
        .enumerate()
        .find_map(|(y, v)| {
            v.iter().enumerate().find_map(|(x, ch)| {
                if *ch == 'S' {
                    Some(Vec2(x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .expect("should have start");

    let mut pipe_loop = vec![start_position.clone()];
    let mut dir_map: HashMap<Direction, Vec2<isize>> = HashMap::new();
    dir_map.insert(Direction::Up, Vec2(0, -1));
    dir_map.insert(Direction::Left, Vec2(-1, 0));
    dir_map.insert(Direction::Right, Vec2(1, 0));
    dir_map.insert(Direction::Down, Vec2(0, 1));

    let mut receiver_map = HashMap::new();
    receiver_map.insert(Direction::Up, vec!['F', '7', '|']);
    receiver_map.insert(Direction::Left, vec!['F', '-', 'L']);
    receiver_map.insert(Direction::Right, vec!['J', '7', '-']);
    receiver_map.insert(Direction::Down, vec!['L', 'J', '|']);

    let mut pipe_map = HashMap::new();
    pipe_map.insert('.', Vec::new());
    pipe_map.insert('|', vec![Direction::Up, Direction::Down]);
    pipe_map.insert('-', vec![Direction::Left, Direction::Right]);
    pipe_map.insert('F', vec![Direction::Right, Direction::Down]);
    pipe_map.insert('7', vec![Direction::Left, Direction::Down]);
    pipe_map.insert('L', vec![Direction::Up, Direction::Right]);
    pipe_map.insert('J', vec![Direction::Up, Direction::Left]);
    pipe_map.insert(
        'S',
        vec![
            Direction::Up,
            Direction::Left,
            Direction::Right,
            Direction::Down,
        ],
    );

    loop {
        let mut should_end = false;

        let last_pos = pipe_loop.last().expect("has last").clone();
        let last_ch = positions[last_pos.1 as usize][last_pos.0 as usize];

        for dir in pipe_map.get(&last_ch).expect("has map") {
            let check_vec = &last_pos + dir_map.get(dir).expect("has vec");

            if check_vec == start_position && pipe_loop.len() > 2 {
                should_end = true;
                break;
            }

            if check_vec.0 >= 0
                && check_vec.1 >= 0
                && check_vec.0 < positions[0].len() as isize
                && check_vec.1 < positions.len() as isize
                && !pipe_loop.contains(&check_vec)
            {
                let check_ch = positions[check_vec.1 as usize][check_vec.0 as usize];
                if receiver_map.get(dir).expect("has").contains(&check_ch) {
                    pipe_loop.push(check_vec);
                    break;
                }
            }
        }

        if should_end {
            break;
        }
    }

    println!("Pipe loop {:?}", pipe_loop.len());

    let mut area = 0;
    for i in 0..pipe_loop.len() {
        let p0 = &pipe_loop[i];
        let p1 = &pipe_loop[(i + 1) % pipe_loop.len()];

        area += p0.0 * p1.1 - p1.0 * p0.1;
    }

    (area.abs() / 2) - (pipe_loop.len() as isize / 2) + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d10_part_2_is_correct_01() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(process(input), 4)
    }

    #[test]
    fn d10_part_2_is_correct_02() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        assert_eq!(process(input), 8)
    }
}
