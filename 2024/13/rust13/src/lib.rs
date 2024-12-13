pub mod part1;
pub mod part2;

#[cfg(all(target_os = "windows", not(test)))]
pub const NEWLINE_OFFSET: usize = 2;
#[cfg(any(not(target_os = "windows"), test))]
pub const NEWLINE_OFFSET: usize = 1;
