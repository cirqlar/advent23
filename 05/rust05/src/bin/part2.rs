use core::ops::Range;
use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

#[derive(Debug)]
struct Transform {
    from_start: u64,
    from_len: u64,
    to_start: u64,
}

fn process(input: &str) -> u64 {
    let mut groups = input.split("\n\n");
    let seeds = groups
        .next()
        .expect("should have seeds")
        .split(' ')
        .skip(1)
        .filter_map(|num| num.parse::<u64>().ok())
        .chunks(2)
        .into_iter()
        .map(|mut c| {
            let start = c.next().expect("should have");
            let length = c.next().expect("should have");
            start..(start + length)
        })
        .collect::<Vec<_>>();

    groups
        .map(|g| {
            g.lines()
                .skip(1)
                .map(|line| {
                    let mut sp = line.split(' ').filter_map(|num| num.parse::<u64>().ok());
                    Transform {
                        to_start: sp.next().expect("should have destination start"),
                        from_start: sp.next().expect("should have source start"),
                        from_len: sp.next().expect("should have range length"),
                    }
                })
                .collect::<Vec<_>>()
        })
        .fold(seeds, |in_ranges: Vec<Range<u64>>, v| {
            let mut transformed = Vec::new();
            let mut left = VecDeque::from(in_ranges);

            while let Some(r) = left.pop_front() {
                let mut found = false;
                for t in v.iter() {
                    let start = r.start.max(t.from_start);
                    let end = r.end.min(t.from_start + t.from_len);
                    if start < end {
                        transformed.push(Range {
                            start: t.to_start + (start - t.from_start),
                            end: t.to_start + (end - t.from_start),
                        });
                        if start > r.start {
                            left.push_back(r.start..start);
                        }
                        if end < r.end {
                            left.push_back(end..r.end)
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    transformed.push(r);
                }
            }

            transformed
        })
        .iter()
        .fold(u64::MAX, |min, current| min.min(current.start))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d05_part_2_is_correct() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(process(input), 46)
    }
}
