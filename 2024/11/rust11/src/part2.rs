use rustc_hash::FxHashMap as HashMap;
use std::str::from_utf8;

use crate::num_digits;

struct Reach {
    // 1 indexed, max 75
    times: usize,
    ends: [usize; 75],
}

impl Reach {
    fn new() -> Self {
        Reach {
            times: 0,
            ends: [0; 75],
        }
    }
}

enum Expansion {
    OneLen([usize; 1]),
    TwoLen([usize; 2]),
}

impl Expansion {
    fn new_one_len(num: usize) -> Expansion {
        Expansion::OneLen([num])
    }

    fn new_two_len(num1: usize, num2: usize) -> Expansion {
        Expansion::TwoLen([num1, num2])
    }

    fn len(&self) -> usize {
        match self {
            Expansion::OneLen(_) => 1,
            Expansion::TwoLen(_) => 2,
        }
    }

    fn get(&self) -> &[usize] {
        match self {
            Expansion::OneLen(a) => a,
            Expansion::TwoLen(a) => a,
        }
    }
}

fn get_nums_for_num(num: usize) -> Expansion {
    if num == 0 {
        return Expansion::new_one_len(1);
    }
    let digits = num_digits(num);

    if digits % 2 == 0 {
        let pow = 10_usize.pow(digits / 2);

        Expansion::new_two_len(num / pow, num % pow)
    } else {
        Expansion::new_one_len(num * 2024)
    }
}

fn add_initial_for_num(num: usize, cache: &mut HashMap<usize, Reach>) {
    cache.entry(num).or_insert_with(|| {
        let mut r = Reach::new();
        r.times = 1;
        r.ends[0] = get_nums_for_num(num).len();
        r
    });
}

fn extend_to_for_num(num: usize, times_left: usize, cache: &mut HashMap<usize, Reach>) {
    let cache_ref = cache.get(&num).unwrap();
    let times = cache_ref.times;
    let zero_ends = get_nums_for_num(num);

    // Eg, if times_left is 25 and times is 24, we want to loop once 24..25
    for time in times..times_left {
        let mut new_ends = 0;

        let child_time = time - 1; // Our 1 is our child's 0

        for end in zero_ends.get() {
            let cache_ref = cache.get(end).unwrap();
            if cache_ref.times <= child_time {
                panic!("We are extending past children for num {} with child {} for time {} and child time {}", num, end, time, child_time);
            }

            new_ends += cache_ref.ends[child_time];
        }

        let cache_mut_ref = cache.get_mut(&num).unwrap();
        cache_mut_ref.ends[time] = new_ends;
        cache_mut_ref.times += 1; // this should be enough to keep our times up to date?
    }
}

fn for_num(num: usize, times_left: usize, cache: &mut HashMap<usize, Reach>) {
    if cache.contains_key(&num) {
        if cache.get(&num).unwrap().times >= times_left {
            return;
        }
    } else {
        add_initial_for_num(num, cache);
        // If times left was 1, we now have our 1. If times left was 0, we shouldn't even be here
        if times_left <= 1 {
            return;
        }
    }

    let zero_ends = get_nums_for_num(num);

    for end in zero_ends.get() {
        for_num(*end, times_left - 1, cache);
    }

    // After we've added to our decendants, we can use theirs to add to ours
    extend_to_for_num(num, times_left, cache);
}

pub fn process(input: &[u8], times: u8) -> usize {
    let stones = input
        .split(|ch| ch.is_ascii_whitespace())
        .filter_map(|num| from_utf8(num).expect("is stringable").parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut cache = HashMap::default();

    let mut res = 0;

    for stone in stones {
        let usize_times: usize = times.into();

        for_num(stone, usize_times, &mut cache);

        res += &cache.get(&stone).unwrap().ends[usize_times - 1];
    }

    res
}
