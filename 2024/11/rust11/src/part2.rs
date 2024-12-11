// use itertools::Itertools;
use std::{collections::HashMap, str::from_utf8};

use crate::num_digits;

struct Reach {
    times: u8,
    count: [usize; 75],
    ends: [Vec<usize>; 75],
}

impl Reach {
    fn new() -> Reach {
        Reach {
            times: 0,
            count: [0; 75],
            ends: [const { Vec::new() }; 75],
        }
    }
}

fn for_num(num: usize, times_left: u8, cache: &mut HashMap<usize, Reach>) {
    if times_left == 0 {
        return;
    }

    let digits = num_digits(num);

    if !cache.contains_key(&num) {
        if digits % 2 == 0 {
            let pow = 10_usize.pow(digits / 2);
            let next_nums = vec![num / pow, num % pow];

            if times_left == 1 {
                cache.entry(num).or_insert_with(|| {
                    let mut r = Reach::new();
                    r.times = 1;
                    r.count[0] = 2;
                    r.ends[0] = next_nums;
                    r
                });
            } else {
                for next in next_nums.iter() {
                    for_num(*next, times_left - 1, cache);
                }

                cache.entry(num).or_insert_with(|| {
                    let mut r = Reach::new();
                    r.times = 1;
                    r.count[0] = 2;
                    r.ends[0] = next_nums.clone();
                    r
                });

                let already_set_times: usize = cache.get(&num).expect("Should exist").times.into();
                let under_us_times_1: usize =
                    cache.get(&next_nums[0]).expect("Should exist").times.into();
                let under_us_times_2 = cache.get(&next_nums[1]).expect("Should exist").times.into();
                let under_us_times = under_us_times_1.max(under_us_times_2);

                for time in already_set_times..=under_us_times {
                    if time >= 75 {
                        break;
                    }
                    let mut under_end_1 =
                        cache.get(&next_nums[0]).expect("Should exist").ends[time - 1].clone();
                    let mut under_end_2 =
                        cache.get(&next_nums[1]).expect("Should exist").ends[time - 1].clone();
                    under_end_1.append(&mut under_end_2);

                    let us = cache.get_mut(&num).expect("Should exist");

                    us.count[time] = under_end_1.len();
                    us.ends[time] = under_end_1;
                    us.times += 1;
                }
            }
        } else {
            let next_num = if num == 0 { 1 } else { num * 2024 };
            if times_left == 1 {
                cache.entry(num).or_insert_with(|| {
                    let mut r = Reach::new();
                    r.times = 1;
                    r.count[0] = 1;
                    r.ends[0] = vec![next_num];
                    r
                });
            } else {
                for_num(next_num, times_left - 1, cache);

                cache.entry(num).or_insert_with(|| {
                    let mut r = Reach::new();
                    r.times = 1;
                    r.count[0] = 1;
                    r.ends[0] = vec![next_num];
                    r
                });

                let already_set_times: usize = cache.get(&num).expect("Should exist").times.into();
                let under_us_times = cache.get(&next_num).expect("Should exist").times.into();

                for time in already_set_times..=under_us_times {
                    if time >= 75 {
                        break;
                    }
                    let under_end =
                        cache.get(&next_num).expect("Should exist").ends[time - 1].clone();
                    let us = cache.get_mut(&num).expect("Should exist");

                    us.count[time] = under_end.len();
                    us.ends[time] = under_end;
                    us.times += 1;
                }
            }
        }
    } else if cache.get(&num).expect("Should exist").times <= times_left {
        return;
    } else {
        let us = cache.get(&num).expect("Should exist");
        let times = us.times;
        let index_up_here: usize = times.into();
        let ends = us.ends[index_up_here].clone();

        for end in ends.iter() {
            for_num(*end, times_left - times, cache);
        }

        for i in 0..(times_left - times) {
            let index_down_here: usize = i.into();
            let mut count = 0;
            let mut new_ends = vec![];

            for end in ends.iter() {
                let temp = cache.get(end).expect("Exists");
                count += temp.count[index_down_here];
                let mut new_end = temp.ends[index_down_here].clone();
                new_ends.append(&mut new_end);
            }

            let us = cache.get_mut(&num).expect("Should exist");
            us.count[index_up_here + index_down_here] = count;
            us.ends[index_up_here + index_down_here] = new_ends;
        }
    }
}

pub fn process(input: &[u8]) -> usize {
    use rayon::prelude::*;

    let mut stones = input
        .split(|ch| ch.is_ascii_whitespace())
        .filter_map(|num| from_utf8(num).expect("is stringable").parse::<usize>().ok())
        .collect::<Vec<_>>();

    for i in 0..75 {
        stones = if stones.len() > 100 {
            println!("Doing turn {} as par", i);
            stones
                .into_par_iter()
                .flat_map_iter(|stone| {
                    let digits = num_digits(stone);
                    if stone == 0 {
                        vec![1].into_iter()
                    } else if digits % 2 == 0 {
                        let pow = 10_usize.pow(digits / 2);
                        vec![stone / pow, stone % pow].into_iter()
                    } else {
                        vec![stone * 2024].into_iter()
                    }
                })
                .collect::<Vec<_>>()
        } else {
            stones
                .into_iter()
                .flat_map(|stone| {
                    let digits = num_digits(stone);
                    if stone == 0 {
                        vec![1].into_iter()
                    } else if digits % 2 == 0 {
                        let pow = 10_usize.pow(digits / 2);
                        vec![stone / pow, stone % pow].into_iter()
                    } else {
                        vec![stone * 2024].into_iter()
                    }
                })
                .collect::<Vec<_>>()
        }
    }

    stones.len()
}

pub fn process_2(input: &[u8]) -> usize {
    let stones = input
        .split(|ch| ch.is_ascii_whitespace())
        .filter_map(|num| from_utf8(num).expect("is stringable").parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut count = 0;
    let mut cache = HashMap::new();

    for stone in stones {
        for_num(stone, 25, &mut cache);

        count += cache.get(&stone).expect("Exists").count[24];
    }

    count
}
