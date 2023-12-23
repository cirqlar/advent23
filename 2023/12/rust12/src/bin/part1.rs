use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use rust12_23::{check_cached, Cache};

fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

fn process(input: &str) -> usize {
    let cache: Cache = Arc::new(RwLock::from(HashMap::new()));
    input
        .lines()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').expect("splable");
            let springs = springs.chars().collect::<Vec<_>>();
            let counts = counts
                .split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>();

            check_cached(&springs, &counts, &cache)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d12_part_1_is_correct_01() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(process(input), 21);
    }
}
