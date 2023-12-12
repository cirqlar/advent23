use std::{
    collections::HashMap,
    iter,
    sync::{Arc, RwLock},
};

use rayon::prelude::*;
use rust12::{check_cached, Cache};

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn process(input: &str) -> usize {
    let cache: Cache = Arc::new(RwLock::from(HashMap::new()));
    input
        .par_lines()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').expect("splable");
            let springs = springs.chars().collect::<Vec<_>>();
            let counts = counts
                .split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>();

            let springs = iter::repeat(springs)
                .take(5)
                .reduce(|mut acc, mut el| {
                    acc.push('?');
                    acc.append(&mut el);
                    acc
                })
                .unwrap();
            let counts = iter::repeat(counts)
                .take(5)
                .flat_map(|a| a.into_iter())
                .collect::<Vec<_>>();

            check_cached(&springs, &counts, &cache)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn d12_part_2_is_correct_01() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(process(input), 525152);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn d12_part_2_is_correct_02(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
