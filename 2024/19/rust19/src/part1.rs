use rustc_hash::{FxBuildHasher, FxHashSet};

fn check_line<'a>(
    input: &'a [u8],
    possibles: &mut FxHashSet<&'a [u8]>,
    min: usize,
    max: usize,
) -> bool {
    if possibles.contains(input) {
        return true;
    }

    for len in min..=max {
        if len > input.len() {
            break;
        }

        if possibles.contains(&input[..len]) {
            let rest = check_line(&input[len..], possibles, min, max);

            if rest {
                possibles.insert(input);
                return true;
            }
        }
    }

    false
}

pub fn process(input: &[u8]) -> usize {
    let split_index = input
        .iter()
        .position(|ch| ch == &b'\n' || ch == &b'\r')
        .expect("multiline");

    let mut min = usize::MAX;
    let mut max = 0;

    let mut towels = FxHashSet::with_capacity_and_hasher(12097, FxBuildHasher);

    input[..split_index]
        .split(|ch| ch == &b',' || ch.is_ascii_whitespace())
        .filter(|towel| !towel.is_empty())
        .for_each(|towel| {
            min = min.min(towel.len());
            max = max.max(towel.len());

            towels.insert(towel);
        });

    let mut possible = 0;

    for pat in input[split_index..]
        .split(|ch| ch.is_ascii_whitespace())
        .filter(|pat| !pat.is_empty())
    {
        if check_line(pat, &mut towels, min, max) {
            possible += 1;
        }
    }

    possible
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
