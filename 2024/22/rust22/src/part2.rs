use rustc_hash::{FxHashMap, FxHashSet};
use std::str::from_utf8;

pub fn process(input: &[u8]) -> usize {
    use rayon::prelude::*;

    let mut overall = FxHashSet::default();

    let allem = input
        .split(|ch| ch.is_ascii_whitespace())
        .filter(|n| !n.is_empty())
        .map(|n| {
            let mut n = from_utf8(n).unwrap().parse::<usize>().unwrap();

            let mut nums = Vec::with_capacity(2001);
            nums.push((n % 10) as i8);

            let mut changes = FxHashMap::default();

            for _ in 0..2000 {
                // 1
                n ^= n * 64;
                n %= 16777216;
                // 2
                let b = n as f64;
                let b = (b / 32.0).floor() as usize;

                n ^= b;
                n %= 16777216;
                // 3
                n ^= n * 2048;
                n %= 16777216;

                nums.push((n % 10) as i8);

                if nums.len() > 4 {
                    let end = nums.len();
                    let check = [
                        nums[end - 4] - nums[end - 5],
                        nums[end - 3] - nums[end - 4],
                        nums[end - 2] - nums[end - 3],
                        nums[end - 1] - nums[end - 2],
                    ];

                    changes.entry(check).or_insert(nums[end - 1]);
                    overall.insert(check);
                }
            }

            changes
        })
        .collect::<Vec<_>>();

    // println!("Finished first part");

    overall
        .par_iter()
        .map(|key| {
            allem
                .par_iter()
                .map(|h| *h.get(key).unwrap_or(&0) as usize)
                .sum()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part2_example.txt");

        let answer = super::process(input);

        assert_eq!(answer, 23);
    }
}
