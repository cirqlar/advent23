pub fn process(input: &[u8]) -> Vec<u8> {
    input
        .iter()
        .filter(|ch| !ch.is_ascii_whitespace())
        .copied()
        .collect()
}

pub fn process_two(input: &[u8]) -> Vec<usize> {
    input
        .iter()
        .enumerate()
        .filter(|(_, ch)| ch == &&b'.')
        .map(|(ind, _)| ind)
        .collect()
}
