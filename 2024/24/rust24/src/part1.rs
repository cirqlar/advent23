use crate::operations::Operation;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::VecDeque, str::from_utf8};

pub fn process(input: &[u8]) -> usize {
    let last_colon = input.len()
        - 1
        - input
            .iter()
            .rev()
            .position(|ch| ch == &b':')
            .expect("Exists");
    let split_position = last_colon
        + input[last_colon..]
            .iter()
            .position(|ch| ch == &b'\n')
            .expect("Exists");

    let mut wires = input[..split_position]
        .split(|ch| ch == &b'\n' || ch == &b'\r')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut key = [0; 3];
            key.copy_from_slice(&line[0..3]);

            (key, line[5] == b'1')
        })
        .collect::<FxHashMap<_, _>>();

    let mut z_wires = FxHashSet::default();

    let mut commands = input[split_position..]
        .split(|ch| ch == &b'\n' || ch == &b'\r')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut a = [0; 3];
            let mut b = [0; 3];
            let mut c = [0; 3];

            let ret = match &line[4..7] {
                b"AND" => {
                    a.copy_from_slice(&line[0..3]);
                    b.copy_from_slice(&line[8..11]);
                    c.copy_from_slice(&line[15..18]);

                    Operation::And(a, b, c)
                }
                b"XOR" => {
                    a.copy_from_slice(&line[0..3]);
                    b.copy_from_slice(&line[8..11]);
                    c.copy_from_slice(&line[15..18]);

                    Operation::Xor(a, b, c)
                }
                b"OR " => {
                    a.copy_from_slice(&line[0..3]);
                    b.copy_from_slice(&line[7..10]);
                    c.copy_from_slice(&line[14..17]);

                    Operation::Orr(a, b, c)
                }
                x => unreachable!("We don't have any other commands {}", from_utf8(x).unwrap()),
            };

            if c[0] == b'z' {
                z_wires.insert(c);
            }

            ret
        })
        .collect::<VecDeque<_>>();

    while let Some(command) = commands.pop_front() {
        if !wires.contains_key(command.get_a()) || !wires.contains_key(command.get_b()) {
            commands.push_back(command);
            continue;
        }

        match &command {
            Operation::And(a, b, c) => {
                let val = *wires.get(a).unwrap() && *wires.get(b).unwrap();
                wires.insert(*c, val);
            }
            Operation::Orr(a, b, c) => {
                let val = *wires.get(a).unwrap() || *wires.get(b).unwrap();
                wires.insert(*c, val);
            }
            Operation::Xor(a, b, c) => {
                let val = *wires.get(a).unwrap() != *wires.get(b).unwrap();
                wires.insert(*c, val);
            }
        }

        if z_wires.iter().all(|wire| wires.contains_key(wire)) {
            break;
        }
    }

    let mut z_wires = z_wires.into_iter().collect::<Vec<_>>();
    z_wires.sort();

    z_wires
        .into_iter()
        // .rev()
        .enumerate()
        .map(|(index, wire)| {
            if *wires.get(&wire).unwrap() {
                2usize.pow(index.try_into().unwrap())
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");

        let answer = super::process(input);

        assert_eq!(answer, 2024);
    }
}
