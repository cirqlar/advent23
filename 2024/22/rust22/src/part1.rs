use std::str::from_utf8;

pub fn process(input: &[u8]) -> usize {
    use rayon::prelude::*;

    input
        .par_split(|ch| ch.is_ascii_whitespace())
        .filter(|n| !n.is_empty())
        .map(|n| {
            let mut n = from_utf8(n).unwrap().parse::<usize>().unwrap();

            for _ in 0..2000 {
                // 1
                n ^= n * 64;
                n %= 16777216;
                // 2
                let b: f64 = n as f64;
                let b: usize = (b / 32.0).floor() as usize;

                n ^= b;
                n %= 16777216;
                // 3
                n ^= n * 2048;
                n %= 16777216;
            }

            n
        })
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");

        let answer = super::process(input);

        assert_eq!(answer, 37327623);
    }
}
