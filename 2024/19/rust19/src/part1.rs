use rustc_hash::{FxBuildHasher, FxHashSet};

fn check_line<'a>(
    input: &'a [u8],
    towels: &FxHashSet<&'a [u8]>,
    cache: &mut FxHashSet<&'a [u8]>,
    min: usize,
    max: usize,
) -> bool {
    if cache.contains(input) || towels.contains(input) {
        return true;
    }

    for len in min..=max {
        if len > input.len() {
            break;
        }

        if towels.contains(&input[..len]) {
            let rest = check_line(&input[len..], towels, cache, min, max);

            if rest {
                cache.insert(input);
                return true;
            }
        }
    }

    false
}

pub fn process(input: &[u8]) -> usize {
    use rayon::prelude::*;

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

    input[split_index..]
        .par_split(|ch| ch.is_ascii_whitespace())
        .filter(|pat| {
            if pat.is_empty() {
                return false;
            }
            check_line(
                pat,
                &towels,
                &mut FxHashSet::with_capacity_and_hasher(50, FxBuildHasher),
                min,
                max,
            )
        })
        .count()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");
        let answer = super::process(input);

        assert_eq!(answer, 6);
    }
}
