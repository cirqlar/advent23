use itertools::Itertools;

use crate::{
    combos::{eight, eleven, five, four, nine, seven, six, ten, three, twelve},
    comp_to_index, index_to_comp, TOTAL_LENGTH,
};

pub fn process(input: &[u8]) -> String {
    let mut connections = [const { None }; TOTAL_LENGTH];

    input
        .split(|ch| ch.is_ascii_whitespace())
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let first = &line[0..2];
            let second = &line[3..5];

            let first_index = comp_to_index(first);
            let second_index = comp_to_index(second);

            let res = connections[first_index].get_or_insert_with(|| Vec::with_capacity(13));
            if !res.contains(&second_index) {
                res.push(second_index);
            }

            let res = connections[second_index].get_or_insert_with(|| Vec::with_capacity(13));
            if !res.contains(&first_index) {
                res.push(first_index);
            }
        });

    let mut largest = vec![];

    for (index, conn) in connections.iter().enumerate().filter(|(_, v)| v.is_some()) {
        let a_ref = conn.as_ref().unwrap();

        if a_ref.iter().any(|n| *n < index) || a_ref.len() <= largest.len() {
            continue;
        }

        let mut cloned = Vec::with_capacity(13);
        for i in (largest.len().max(3)..=a_ref.len().min(12)).rev() {
            match i {
                12 => {
                    if let Some(tuple) = a_ref
                        .iter()
                        .tuple_combinations()
                        .find(|tuple| twelve(*tuple, &connections))
                    {
                        cloned.push(index);
                        cloned.push(*tuple.0);
                        cloned.push(*tuple.1);
                        cloned.push(*tuple.2);
                        cloned.push(*tuple.3);
                        cloned.push(*tuple.4);
                        cloned.push(*tuple.5);
                        cloned.push(*tuple.6);
                        cloned.push(*tuple.7);
                        cloned.push(*tuple.8);
                        cloned.push(*tuple.9);
                        cloned.push(*tuple.10);
                        cloned.push(*tuple.11);
                        break;
                    }
                }
                11 => {
                    if let Some(tuple) = a_ref
                        .iter()
                        .tuple_combinations()
                        .find(|tuple| eleven(*tuple, &connections))
                    {
                        cloned.push(index);
                        cloned.push(*tuple.0);
                        cloned.push(*tuple.1);
                        cloned.push(*tuple.2);
                        cloned.push(*tuple.3);
                        cloned.push(*tuple.4);
                        cloned.push(*tuple.5);
                        cloned.push(*tuple.6);
                        cloned.push(*tuple.7);
                        cloned.push(*tuple.8);
                        cloned.push(*tuple.9);
                        cloned.push(*tuple.10);
                        break;
                    }
                }
                10 => {
                    if let Some(tuple) = a_ref
                        .iter()
                        .tuple_combinations()
                        .find(|tuple| ten(*tuple, &connections))
                    {
                        cloned.push(index);
                        cloned.push(*tuple.0);
                        cloned.push(*tuple.1);
                        cloned.push(*tuple.2);
                        cloned.push(*tuple.3);
                        cloned.push(*tuple.4);
                        cloned.push(*tuple.5);
                        cloned.push(*tuple.6);
                        cloned.push(*tuple.7);
                        cloned.push(*tuple.8);
                        cloned.push(*tuple.9);
                        break;
                    }
                }
                9 => {
                    if let Some(tuple) = a_ref
                        .iter()
                        .tuple_combinations()
                        .find(|tuple| nine(*tuple, &connections))
                    {
                        cloned.push(index);
                        cloned.push(*tuple.0);
                        cloned.push(*tuple.1);
                        cloned.push(*tuple.2);
                        cloned.push(*tuple.3);
                        cloned.push(*tuple.4);
                        cloned.push(*tuple.5);
                        cloned.push(*tuple.6);
                        cloned.push(*tuple.7);
                        cloned.push(*tuple.8);
                        break;
                    }
                }
                8 => {
                    if let Some(tuple) = a_ref
                        .iter()
                        .tuple_combinations()
                        .find(|tuple| eight(*tuple, &connections))
                    {
                        cloned.push(index);
                        cloned.push(*tuple.0);
                        cloned.push(*tuple.1);
                        cloned.push(*tuple.2);
                        cloned.push(*tuple.3);
                        cloned.push(*tuple.4);
                        cloned.push(*tuple.5);
                        cloned.push(*tuple.6);
                        cloned.push(*tuple.7);
                        break;
                    }
                }
                7 => {
                    if let Some(tuple) = a_ref
                        .iter()
                        .tuple_combinations()
                        .find(|tuple| seven(*tuple, &connections))
                    {
                        cloned.push(index);
                        cloned.push(*tuple.0);
                        cloned.push(*tuple.1);
                        cloned.push(*tuple.2);
                        cloned.push(*tuple.3);
                        cloned.push(*tuple.4);
                        cloned.push(*tuple.5);
                        cloned.push(*tuple.6);
                        break;
                    }
                }
                6 => {
                    if let Some(tuple) = a_ref
                        .iter()
                        .tuple_combinations()
                        .find(|tuple| six(*tuple, &connections))
                    {
                        cloned.push(index);
                        cloned.push(*tuple.0);
                        cloned.push(*tuple.1);
                        cloned.push(*tuple.2);
                        cloned.push(*tuple.3);
                        cloned.push(*tuple.4);
                        cloned.push(*tuple.5);
                        break;
                    }
                }
                5 => {
                    if let Some(tuple) = a_ref
                        .iter()
                        .tuple_combinations()
                        .find(|tuple| five(*tuple, &connections))
                    {
                        cloned.push(index);
                        cloned.push(*tuple.0);
                        cloned.push(*tuple.1);
                        cloned.push(*tuple.2);
                        cloned.push(*tuple.3);
                        cloned.push(*tuple.4);
                        break;
                    }
                }
                4 => {
                    if let Some(tuple) = a_ref
                        .iter()
                        .tuple_combinations()
                        .find(|tuple| four(*tuple, &connections))
                    {
                        cloned.push(index);
                        cloned.push(*tuple.0);
                        cloned.push(*tuple.1);
                        cloned.push(*tuple.2);
                        cloned.push(*tuple.3);
                        break;
                    }
                }
                3 => {
                    if let Some(tuple) = a_ref
                        .iter()
                        .tuple_combinations()
                        .find(|tuple| three(*tuple, &connections))
                    {
                        cloned.push(index);
                        cloned.push(*tuple.0);
                        cloned.push(*tuple.1);
                        cloned.push(*tuple.2);
                        break;
                    }
                }
                x => unimplemented!("haven't implemented for {}", x),
            }
        }

        if cloned.len() > largest.len() {
            cloned.sort();
            largest = cloned;
        }
    }

    largest
        .into_iter()
        .map(index_to_comp)
        .map(|chs| chs.into_iter().join(""))
        .join(",")
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");

        let answer = super::process(input);

        assert_eq!(&answer, "co,de,ka,ta");
    }
}
