use itertools::Itertools;
use std::str::from_utf8;

use crate::num_digits;

pub fn process(input: &[u8], times: u8) -> usize {
    let mut stones = input
        .split(|ch| ch.is_ascii_whitespace())
        .filter_map(|num| from_utf8(num).expect("is stringable").parse::<usize>().ok())
        .collect::<Vec<_>>();

    for _ in 0..times {
        // do stuff

        let mut next_half = None;

        stones = stones
            .into_iter()
            .batching(|it| {
                if next_half.is_some() {
                    next_half.take()
                } else if let Some(next) = it.next() {
                    let digits = num_digits(next);
                    if next == 0 {
                        Some(1)
                    } else if digits % 2 == 0 {
                        let pow = 10_usize.pow(digits / 2);
                        next_half = Some(next % pow);
                        Some(next / pow)
                    } else {
                        Some(next * 2024)
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
    }

    stones.len()
}

pub fn process_return(input: &[u8], times: u8) -> Vec<usize> {
    let mut stones = input
        .split(|ch| ch.is_ascii_whitespace())
        .filter_map(|num| from_utf8(num).expect("is stringable").parse::<usize>().ok())
        .collect::<Vec<_>>();

    for _ in 0..times {
        // do stuff

        let mut next_half = None;

        stones = stones
            .into_iter()
            .batching(|it| {
                if next_half.is_some() {
                    next_half.take()
                } else if let Some(next) = it.next() {
                    let digits = num_digits(next);
                    if next == 0 {
                        Some(1)
                    } else if digits % 2 == 0 {
                        let pow = 10_usize.pow(digits / 2);
                        next_half = Some(next % pow);
                        Some(next / pow)
                    } else {
                        Some(next * 2024)
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
    }

    stones
}
