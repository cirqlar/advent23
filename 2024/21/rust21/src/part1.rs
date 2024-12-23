use crate::{
    arrow_to_num,
    fns::{arrows_to_path_to_arrows, num_to_xy, numpad_successors, path_to_arrows},
    num_to_num, ARROW_MAP, NEWLINE_OFFSET, NUMPAD_MAP,
};
use pathfinding::prelude::astar_bag_collect;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::str::from_utf8;

pub fn process(input: &[u8]) -> usize {
    let input_grid_size = 4 + NEWLINE_OFFSET;

    // let mut cache = ;
    // let mut total = 0;

    (0..5)
        .into_par_iter()
        .map(|i| {
            let start = i * input_grid_size;
            let end = start + 4;

            let mut prev = b'A';

            let mut paths = vec![];
            let mut elon_paths = vec![];
            // let mut ends = vec![];

            for ch in input[start..end].iter() {
                let exy = num_to_xy(*ch);
                let shortests = astar_bag_collect(
                    &prev,
                    numpad_successors,
                    |n| {
                        let nxy = num_to_xy(*n);

                        nxy.0.abs_diff(exy.0) + nxy.1.abs_diff(exy.1)
                    },
                    |n| n == ch,
                )
                .expect("can path");

                if paths.is_empty() {
                    shortests.0.into_iter().for_each(|path| {
                        let mut n_path = path_to_arrows(path);
                        n_path.push(b'A');

                        paths.push(n_path);
                    });
                } else {
                    for extension in shortests.0 {
                        let mut extension = path_to_arrows(extension);
                        extension.push(b'A');

                        for path in paths.iter() {
                            let mut n_path = path.clone();

                            for nch in extension.iter() {
                                n_path.push(*nch);
                            }

                            elon_paths.push(n_path);
                        }
                    }

                    std::mem::swap(&mut paths, &mut elon_paths);
                    elon_paths.clear();
                }

                prev = *ch;
            }

            let mut min = usize::MAX;

            for path in paths {
                min = arrows_to_path_to_arrows(path, 2, &mut FxHashMap::default())
                    .len()
                    .min(min);
            }
            println!("Finished {}", i);

            from_utf8(&input[start..(end - 1)])
                .unwrap()
                .parse::<usize>()
                .unwrap()
                * (min - 6)
        })
        .sum()

    // total
}

pub fn process_failed(input: &[u8]) -> usize {
    let grid_size = 4 + NEWLINE_OFFSET;

    let mut total = 0;

    for i in 0..5 {
        let start = i * grid_size;
        let end = start + 4;

        println!("{}", from_utf8(&input[start..end]).unwrap());

        let mut previous = b'A';
        let first_stage = input[start..end]
            .iter()
            .flat_map(|ch| {
                let index = num_to_num(previous) * 11 + num_to_num(*ch);
                previous = *ch;

                NUMPAD_MAP[index]
            })
            .copied()
            .collect::<Vec<_>>();

        println!("{}", from_utf8(&first_stage[..]).unwrap());

        let mut previous = b'A';
        let second_stage = first_stage
            .into_iter()
            .flat_map(|ch| {
                let index = arrow_to_num(previous) * 5 + arrow_to_num(ch);
                previous = ch;

                ARROW_MAP[index]
            })
            .copied()
            .collect::<Vec<_>>();

        println!("{}", from_utf8(&second_stage[..]).unwrap());

        let mut previous = b'A';
        let third_stage = second_stage
            .into_iter()
            .flat_map(|ch| {
                let index = arrow_to_num(previous) * 5 + arrow_to_num(ch);
                previous = ch;

                ARROW_MAP[index]
            })
            .copied()
            .collect::<Vec<_>>();

        // let a = from_utf8(&input[start..end]).expect("can string");

        println!("{}", from_utf8(&third_stage[..]).unwrap());

        println!("{}", third_stage.len());

        total += from_utf8(&input[start..(end - 1)])
            .expect("can string")
            .parse::<usize>()
            .expect("can num")
            * third_stage.len();
    }

    total
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");

        let answer = super::process(input);

        assert_eq!(answer, 126384);
    }
}
