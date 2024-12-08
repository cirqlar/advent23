pub mod parse;
pub mod part1;
pub mod part2;

#[cfg(target_os = "windows")]
pub const NEWLINE_OFFSET: usize = 2;
#[cfg(not(target_os = "windows"))]
pub const NEWLINE_OFFSET: usize = 1;

#[inline(always)]
pub fn check_and_count(index: usize, check: &mut [bool], count: &mut usize) {
    if !check[index] {
        check[index] = true;
        *count += 1;
    }
}
