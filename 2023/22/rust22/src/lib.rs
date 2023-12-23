use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec3 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Vec3 {
    pub fn new(x: usize, y: usize, z: usize) -> Vec3 {
        Vec3 { x, y, z }
    }
}

impl Ord for Vec3 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.z.cmp(&other.z) {
            std::cmp::Ordering::Equal => match self.y.cmp(&other.y) {
                std::cmp::Ordering::Equal => self.x.cmp(&other.x),
                x => x,
            },
            x => x,
        }
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Vec3 {
    fn from(value: &str) -> Self {
        let mut input = value.split(',').map(|s| s.parse::<usize>().unwrap());
        Vec3 {
            x: input.next().unwrap(),
            y: input.next().unwrap(),
            z: input.next().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Brick {
    pub start: Vec3,
    pub end: Vec3,
}

pub fn bricks_from_str(input: &str) -> Vec<Brick> {
    let mut bricks = input
        .lines()
        .map(|line| {
            let pos = line.split_once('~').unwrap();
            Brick {
                start: pos.0.into(),
                end: pos.1.into(),
            }
        })
        .collect::<Vec<_>>();

    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    bricks
}

pub fn to_stacked_bricks(bricks: Vec<Brick>) -> Vec<(Brick, Vec<Brick>)> {
    let mut stack: BTreeMap<usize, Vec<(Brick, Vec<Brick>)>> = BTreeMap::new();

    for mut brick in bricks {
        let mut insert_at = brick.start.z;
        let mut supported_by = Vec::new();
        for i in (1..brick.start.z).rev() {
            if let Some(vv) = stack.get(&i) {
                let mut should_break = false;
                for (b, _) in vv {
                    if (brick.start.x <= b.end.x && brick.end.x >= b.start.x)
                        && (brick.start.y <= b.end.y && brick.end.y >= b.start.y)
                    {
                        insert_at = insert_at.max(b.end.z + 1);
                        supported_by.push(b.clone());
                        should_break = true;
                    }
                }
                if should_break {
                    break;
                }
            }
            insert_at = i;
        }

        let bsz = brick.start.z;
        let bez = brick.end.z;
        brick.start.z = insert_at;
        brick.end.z = insert_at + (bez - bsz);

        for i in brick.start.z..=brick.end.z {
            stack
                .entry(i)
                .and_modify(|vv| vv.push((brick.clone(), supported_by.clone())))
                .or_insert(vec![(brick.clone(), supported_by.clone())]);
        }
    }

    let bricks = stack
        .values()
        .flat_map(|v| v.iter())
        .cloned()
        .collect::<BTreeSet<_>>();

    bricks.into_iter().collect_vec()
}

pub fn get_undisintigratable(bricks: &[(Brick, Vec<Brick>)]) -> Vec<&Brick> {
    bricks
        .iter()
        .filter_map(|(_, vs)| if vs.len() == 1 { Some(&vs[0]) } else { None })
        .unique()
        .collect_vec()
}
