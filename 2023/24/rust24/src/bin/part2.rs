use std::ops::{Add, Mul};

use itertools::Itertools;
use z3::{
    ast::{Ast, Int, Real},
    Config, Context, Solver,
};

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3 {
    pub fn new(x: i64, y: i64, z: i64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

impl From<&str> for Vec3 {
    fn from(value: &str) -> Self {
        let mut input = value.split(',').map(|s| s.trim().parse::<i64>().unwrap());
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

fn process(input: &str) -> i64 {
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

    let config = Config::new();
    let ctx = Context::new(&config);
    let solver = Solver::new(&ctx);

    let rx = Real::new_const(&ctx, "rx");
    let ry = Real::new_const(&ctx, "ry");
    let rz = Real::new_const(&ctx, "rz");
    let rvx = Real::new_const(&ctx, "rvx");
    let rvy = Real::new_const(&ctx, "rvy");
    let rvz = Real::new_const(&ctx, "rvz");
    let sum = Real::new_const(&ctx, "sum");
    solver.assert(&(&rx).add(&ry).add(&rz)._eq(&sum));

    for (i, hail) in hails.iter().enumerate().take(3) {
        let ti = Real::new_const(&ctx, format!("t{i}"));
        let xi = Real::from_int(&Int::from_i64(&ctx, hail.pos.x));
        let yi = Real::from_int(&Int::from_i64(&ctx, hail.pos.y));
        let zi = Real::from_int(&Int::from_i64(&ctx, hail.pos.z));
        let vxi = Real::from_int(&Int::from_i64(&ctx, hail.vel.x));
        let vyi = Real::from_int(&Int::from_i64(&ctx, hail.vel.y));
        let vzi = Real::from_int(&Int::from_i64(&ctx, hail.vel.z));
        solver.assert(&(&vxi).mul(&ti).add(&xi)._eq(&(&rvx).mul(&ti).add(&rx)));
        solver.assert(&(&vyi).mul(&ti).add(&yi)._eq(&(&rvy).mul(&ti).add(&ry)));
        solver.assert(&(&vzi).mul(&ti).add(&zi)._eq(&(&rvz).mul(&ti).add(&rz)));
    }

    solver.check();
    let model = solver.get_model().unwrap();

    let sum = model.get_const_interp(&sum).unwrap();
    sum.as_real().unwrap().0
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

        assert_eq!(process(input), 47);
    }
}
