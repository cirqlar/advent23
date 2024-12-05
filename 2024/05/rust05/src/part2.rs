use std::{cmp::Ordering, str::from_utf8};

use crate::parse::{RuleMap, NEWLINE_OFFSET};

pub fn process(input: &[u8], split_line_number: usize, rule_map: &RuleMap<i32>) -> i32 {
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
                            .parse::<i32>()
                            .expect("Numberable")
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|line| {
            for index in 0..(line.len() - 1) {
                for other_index in (index + 1)..line.len() {
                    if let Some((befores, _)) = rule_map.get(&line[index]) {
                        if befores.contains(&line[other_index]) {
                            return true;
                        }
                    }
                    if let Some((_, afters)) = rule_map.get(&line[other_index]) {
                        if afters.contains(&line[index]) {
                            return true;
                        }
                    }
                }
            }

            false
        })
        .map(|mut line| {
            line.sort_by(|a, b| {
                if let Some((befores, afters)) = rule_map.get(a) {
                    if befores.contains(b) {
                        return Ordering::Greater;
                    }
                    if afters.contains(b) {
                        return Ordering::Less;
                    }
                }
                if let Some((befores, afters)) = rule_map.get(b) {
                    if befores.contains(a) {
                        return Ordering::Less;
                    }
                    if afters.contains(a) {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            });

            let index = line.len() / 2;
            line[index]
        })
        .sum()
}

pub fn process_par(input: &[u8], split_line_number: usize, rule_map: &RuleMap<i32>) -> i32 {
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
                            .parse::<i32>()
                            .expect("Numberable")
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|line| {
            for index in 0..(line.len() - 1) {
                for other_index in (index + 1)..line.len() {
                    if let Some((befores, _)) = rule_map.get(&line[index]) {
                        if befores.contains(&line[other_index]) {
                            return true;
                        }
                    }
                    if let Some((_, afters)) = rule_map.get(&line[other_index]) {
                        if afters.contains(&line[index]) {
                            return true;
                        }
                    }
                }
            }

            false
        })
        .map(|mut line| {
            line.sort_by(|a, b| {
                if let Some((befores, afters)) = rule_map.get(a) {
                    if befores.contains(b) {
                        return Ordering::Greater;
                    }
                    if afters.contains(b) {
                        return Ordering::Less;
                    }
                }
                if let Some((befores, afters)) = rule_map.get(b) {
                    if befores.contains(a) {
                        return Ordering::Less;
                    }
                    if afters.contains(a) {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            });

            let index = line.len() / 2;
            line[index]
        })
        .sum()
}

pub fn process_par_2(input: &[u8], split_line_number: usize, rule_map: &RuleMap<i32>) -> i32 {
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
                line.par_chunks(3)
                    .map(|chunk| {
                        from_utf8(&chunk[0..2])
                            .expect("Stringable")
                            .parse::<i32>()
                            .expect("Numberable")
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|line| {
            for index in 0..(line.len() - 1) {
                for other_index in (index + 1)..line.len() {
                    if let Some((befores, _)) = rule_map.get(&line[index]) {
                        if befores.contains(&line[other_index]) {
                            return true;
                        }
                    }
                    if let Some((_, afters)) = rule_map.get(&line[other_index]) {
                        if afters.contains(&line[index]) {
                            return true;
                        }
                    }
                }
            }

            false
        })
        .map(|mut line| {
            line.sort_by(|a, b| {
                if let Some((befores, afters)) = rule_map.get(a) {
                    if befores.contains(b) {
                        return Ordering::Greater;
                    }
                    if afters.contains(b) {
                        return Ordering::Less;
                    }
                }
                if let Some((befores, afters)) = rule_map.get(b) {
                    if befores.contains(a) {
                        return Ordering::Less;
                    }
                    if afters.contains(a) {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            });

            let index = line.len() / 2;
            line[index]
        })
        .sum()
}
