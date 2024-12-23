pub mod combos;
pub mod part1;
pub mod part2;

#[cfg(target_os = "windows")]
pub const NEWLINE_OFFSET: usize = 2;
#[cfg(not(target_os = "windows"))]
pub const NEWLINE_OFFSET: usize = 1;

pub const LARGEST_SINGLE_INDEX: usize = (b'z' - b'a') as usize;
pub const TOTAL_LENGTH: usize = LARGEST_SINGLE_INDEX * 100 + LARGEST_SINGLE_INDEX + 1;

pub fn comp_to_index(comp: &[u8]) -> usize {
    ((comp[0] - b'a') as usize) * 100 + ((comp[1] - b'a') as usize)
}

pub fn index_to_comp(index: usize) -> [char; 2] {
    let a = ((index / 100) as u8) + b'a';
    let b = ((index % 100) as u8) + b'a';

    [a as char, b as char]
}
