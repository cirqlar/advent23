use crate::NEWLINE_OFFSET;
use itertools::Itertools;

pub fn process(input: &[u8]) -> usize {
    let full_width = 5 + NEWLINE_OFFSET;
    let full_size = full_width * 7;
    let with_following_space = full_size + NEWLINE_OFFSET;

    let ks_and_ls = input
        .chunks(with_following_space)
        .map(|kl| {
            let mut lens = [0; 5];
            if kl[0] == b'#' {
                for (i, el) in lens.iter_mut().enumerate() {
                    *el = kl
                        .iter()
                        .skip(i)
                        .step_by(full_width)
                        .position(|ch| ch == &b'.')
                        .unwrap()
                        - 1;
                }

                (0, lens)
            } else {
                for (i, el) in lens.iter_mut().enumerate() {
                    *el = 6 - kl
                        .iter()
                        .skip(i)
                        .step_by(full_width)
                        .position(|ch| ch == &b'#')
                        .unwrap();
                }
                (1, lens)
            }
        })
        .collect::<Vec<_>>();

    let mut locks = Vec::with_capacity(100);
    let mut keys = Vec::with_capacity(100);

    ks_and_ls.into_iter().for_each(|(ty, lens)| {
        if ty == 0 {
            locks.push(lens);
        } else {
            keys.push(lens);
        }
    });

    locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(l, k)| {
            // println!("Lock {:?} and Key {:?}", l, k);
            for (lv, kv) in l.iter().zip(k) {
                if lv + kv > 5 {
                    return false;
                }
            }
            true
        })
        .count()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");

        let answer = super::process(input);

        assert_eq!(answer, 2024);
    }
}
