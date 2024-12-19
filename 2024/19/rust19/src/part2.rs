use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

fn check_line<'a>(
    input: &'a [u8],
    towels: &FxHashSet<&'a [u8]>,
    cache: &mut FxHashMap<&'a [u8], usize>,
    min: usize,
    max: usize,
) -> usize {
    if cache.contains_key(input) {
        return *cache.get(input).unwrap();
    }

    let mut total = 0;

    for len in min..=max {
        if len > input.len() {
            break;
        }

        if towels.contains(&input[..len]) {
            if len == input.len() {
                total += 1;
                continue;
            }

            total += check_line(&input[len..], towels, cache, min, max);
        }
    }

    if total > 0 {
        cache.insert(input, total);
    }

    total
}

pub fn process(input: &[u8]) -> usize {
    let split_index = input
        .iter()
        .position(|ch| ch == &b'\n' || ch == &b'\r')
        .expect("multiline");

    let mut min = usize::MAX;
    let mut max = 0;

    let towels = input[..split_index]
        .split(|ch| ch == &b',' || ch.is_ascii_whitespace())
        .filter(|towel| !towel.is_empty())
        .inspect(|towel| {
            min = min.min(towel.len());
            max = max.max(towel.len());
        })
        .collect::<FxHashSet<_>>();

    let mut total = 0;
    let mut cache = FxHashMap::with_capacity_and_hasher(14656, FxBuildHasher);

    for pat in input[split_index..]
        .split(|ch| ch.is_ascii_whitespace())
        .filter(|pat| !pat.is_empty())
    {
        total += check_line(pat, &towels, &mut cache, min, max);
    }

    total
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");
        let answer = super::process(input);

        assert_eq!(answer, 16);
    }
}
