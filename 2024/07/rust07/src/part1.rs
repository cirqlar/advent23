use std::{collections::VecDeque, str::from_utf8};

pub fn process(input: &[u8]) -> u64 {
    use rayon::prelude::*;

    input
        .par_split(|ch| ch == &b'\n' || ch == &b'\r')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            let mut met = line
                .split(|ch| ch == &b':' || ch.is_ascii_whitespace())
                .filter(|num| !num.is_empty());

            let total: u64 = from_utf8(met.next().unwrap())
                .expect("is stringable")
                .parse()
                .expect("is numberable");
            let nums = met
                .map(|num| {
                    from_utf8(num)
                        .expect("num stringable")
                        .parse::<u64>()
                        .expect("num numberable")
                })
                .collect::<Vec<_>>();

            Some((total, nums))
        })
        .map(|(total, nums)| {
            // check even practical? sum is equal to or less and product is equal to or greater
            let sum: u64 = nums.iter().sum();
            let prod: u64 = nums.iter().product();

            // check if we can do a simple one
            if sum == total {
                return total;
            }
            if prod == total {
                return total;
            }

            if sum > total && prod < total {
                return 0;
            }

            // check possible
            let mut options = VecDeque::from([(total, 0)]);
            let len = nums.len();

            loop {
                let Some((tot, skip)) = options.pop_front() else {
                    break;
                };

                let consider = nums[len - 1 - skip];

                // Check Last Element
                if skip == len - 1 {
                    if tot == consider {
                        return total;
                    }

                    continue;
                }

                // Check not even possible
                if tot < consider {
                    continue;
                }

                // Check Add
                let rem = tot - consider;
                if rem > 0 {
                    options.push_back((rem, skip + 1));
                }

                // Check Mult
                let rem = tot % consider;
                if rem == 0 {
                    options.push_back((tot / consider, skip + 1));
                }
            }

            0
        })
        .sum()
}
