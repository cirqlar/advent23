use std::str::from_utf8;

use crate::parse::{RuleMap, RuleMap2, ARR_WIDTH, ARR_WIDTH_U16, NEWLINE_OFFSET};

pub fn process(input: &[u8], split_line_number: usize, rule_map: &RuleMap<u16>) -> u16 {
    let pages = &input[((5 + NEWLINE_OFFSET) * (split_line_number - 1))..];

    pages
        .chunk_by(|a, b| !a.is_ascii_whitespace() && !b.is_ascii_whitespace())
        .filter_map(|line| {
            if line.len() < 2 {
                return None;
            }

            Some(
                line.chunks(3)
                    .map(|chunk| {
                        from_utf8(&chunk[0..2])
                            .expect("Stringable")
                            .parse::<u16>()
                            .expect("Numberable")
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|line| {
            for index in 0..(line.len() - 1) {
                for other_index in (index + 1)..line.len() {
                    if let Some(befores) = rule_map.get(&line[index]) {
                        if befores.contains(&line[other_index]) {
                            return false;
                        }
                    }
                }
            }

            true
        })
        .map(|line| {
            let index = line.len() / 2;
            line[index]
        })
        .sum()
}

pub fn process_par(input: &[u8], split_line_number: usize, rule_map: &RuleMap<u16>) -> u16 {
    use rayon::prelude::*;

    let pages = &input[((5 + NEWLINE_OFFSET) * (split_line_number - 1))..];

    pages
        .chunk_by(|a, b| !a.is_ascii_whitespace() && !b.is_ascii_whitespace())
        .par_bridge()
        .filter_map(|line| {
            if line.len() < 2 {
                return None;
            }

            Some(
                line.chunks(3)
                    .map(|chunk| {
                        from_utf8(&chunk[0..2])
                            .expect("Stringable")
                            .parse::<u16>()
                            .expect("Numberable")
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|line| {
            for index in 0..(line.len() - 1) {
                for other_index in (index + 1)..line.len() {
                    if let Some(befores) = rule_map.get(&line[index]) {
                        if befores.contains(&line[other_index]) {
                            return false;
                        }
                    }
                }
            }

            true
        })
        .map(|line| {
            let index = line.len() / 2;
            line[index]
        })
        .sum()
}

pub fn process_par_2(input: &[u8], split_line_number: usize, rule_map: &RuleMap<u16>) -> u16 {
    use rayon::prelude::*;

    let pages = &input[((5 + NEWLINE_OFFSET) * (split_line_number - 1))..];

    pages
        .chunk_by(|a, b| !a.is_ascii_whitespace() && !b.is_ascii_whitespace())
        .filter(|line| line.len() > 1)
        .par_bridge()
        .map(|line| {
            line.par_chunks(3)
                .map(|chunk| {
                    from_utf8(&chunk[0..2])
                        .expect("Stringable")
                        .parse::<u16>()
                        .expect("Numberable")
                })
                .collect::<Vec<_>>()
        })
        .map(|line| {
            for index in 0..(line.len() - 1) {
                for other_index in (index + 1)..line.len() {
                    if let Some(befores) = rule_map.get(&line[index]) {
                        if befores.contains(&line[other_index]) {
                            return 0;
                        }
                    }
                }
            }

            let index = line.len() / 2;
            line[index]
        })
        .sum()
}

pub fn process_par_2_vec(input: &[u8], split_line_number: usize, rule_map: &RuleMap2<u16>) -> u16 {
    use rayon::prelude::*;

    let pages = &input[((5 + NEWLINE_OFFSET) * (split_line_number - 1))..];

    pages
        .chunk_by(|a, b| !a.is_ascii_whitespace() && !b.is_ascii_whitespace())
        .filter(|line| line.len() > 1)
        .par_bridge()
        .map(|line| {
            line.par_chunks(3)
                .map(|chunk| {
                    from_utf8(&chunk[0..2])
                        .expect("Stringable")
                        .parse::<u16>()
                        .expect("Numberable")
                })
                .collect::<Vec<_>>()
        })
        .map(|line| {
            for index in 0..(line.len() - 1) {
                for other_index in (index + 1)..line.len() {
                    let starting_index: usize = ((line[index] - 10) * ARR_WIDTH_U16).into();
                    if rule_map[starting_index..(starting_index + ARR_WIDTH)]
                        .contains(&Some(line[other_index]))
                    {
                        return 0;
                    }

                    // if let Some(befores) = rule_map.get(&line[index]) {
                    //     if befores.contains(&line[other_index]) {
                    //         return 0;
                    //     }
                    // }
                }
            }

            let index = line.len() / 2;
            line[index]
        })
        .sum()
}
