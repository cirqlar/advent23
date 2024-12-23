use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::{comp_to_index, LARGEST_SINGLE_INDEX, TOTAL_LENGTH};

pub fn process(input: &[u8]) -> usize {
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

    let first_t_index = ((b't' - b'a') as usize) * 100;
    let last_t_index = first_t_index + LARGEST_SINGLE_INDEX;

    let mut cache = FxHashSet::default();

    connections
        .iter()
        .enumerate()
        .skip(first_t_index)
        .take(last_t_index - first_t_index + 1)
        .filter(|(_, opt)| opt.is_some())
        .map(|(ind, cons)| {
            cons.as_ref()
                .unwrap()
                .iter()
                .tuple_combinations()
                .filter(|(a, b)| {
                    if connections[**a].as_ref().unwrap().contains(*b) {
                        let mut ems = [ind, **a, **b];
                        ems.sort();

                        if cache.contains(&ems) {
                            false
                        } else {
                            cache.insert(ems);
                            true
                        }
                    } else {
                        false
                    }
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");

        let answer = super::process(input);

        assert_eq!(answer, 7);
    }
}
