fn main() {
    let input = include_str!("../../../input/part1.txt");

    let answer = process(input);

    println!("Answer {answer}");
}

#[derive(Debug)]
struct Range {
    from_start: u32,
    from_len: u32,
    to_start: u32,
}

fn process(input: &str) -> u32 {
    let mut groups = input.split("\n\n");
    let seeds = groups.next().expect("should have seeds");
    let seeds = seeds
        .split(": ")
        .nth(1)
        .expect("should have seed numbers")
        .split(' ')
        .filter_map(|num| num.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let maps = groups
        .map(|g| {
            g.lines()
                .skip(1)
                .map(|line| {
                    let mut sp = line.split(' ').filter_map(|num| num.parse::<u32>().ok());
                    Range {
                        to_start: sp.next().expect("should have destination start"),
                        from_start: sp.next().expect("should have source start"),
                        from_len: sp.next().expect("should have range length"),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |seed, map| {
                for r in map {
                    let diff = (seed as i64) - (r.from_start as i64);
                    if diff >= 0 && diff < (r.from_len as i64) {
                        let diff: u32 = diff.try_into().expect("should be small enough");
                        return r.to_start + diff;
                    }
                }
                seed
            })
        })
        .fold(u32::MAX, |min, current| min.min(current))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d05_part_1_is_correct() {
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

        assert_eq!(process(input), 35)
    }
}
