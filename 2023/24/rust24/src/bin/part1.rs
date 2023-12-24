use std::ops::{Add, Mul};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input, 200000000000000.0, 400000000000000.0);

    println!("Answer {answer}");
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// impl Add for &Vec3 {
//     type Output = Vec3;

//     fn add(self, rhs: Self) -> Self::Output {
//         Vec3 {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z,
//         }
//     }
// }

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl From<&str> for Vec3 {
    fn from(value: &str) -> Self {
        let mut input = value.split(',').map(|s| s.trim().parse::<f64>().unwrap());
        Vec3 {
            x: input.next().unwrap(),
            y: input.next().unwrap(),
            z: input.next().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Hail {
    pos: Vec3,
    vel: Vec3,
}

fn process(input: &str, min: f64, max: f64) -> usize {
    let hails = input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            Hail {
                pos: pos.into(),
                vel: vel.into(),
            }
        })
        .collect_vec();

    hails
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            let tb = ((a.vel.x * (b.pos.y - a.pos.y)) + (a.vel.y * (a.pos.x - b.pos.x)))
                / ((a.vel.y * b.vel.x) - (a.vel.x * b.vel.y));

            if tb < 0.0 {
                return false;
            }
            let bpos = b.pos + (b.vel * tb);

            if bpos.x < min || bpos.x > max || bpos.y < min || bpos.y > max {
                return false;
            }

            let ta = (b.pos.y - a.pos.y + (b.vel.y * tb)) / a.vel.y;

            if ta < 0.0 {
                return false;
            }
            true
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d24_part_1_is_correct() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

        assert_eq!(process(input, 7.0, 27.0), 2);
    }
}
