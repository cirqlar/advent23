pub mod part1;

#[cfg(target_os = "windows")]
pub const NEWLINE_OFFSET: usize = 2;
#[cfg(not(target_os = "windows"))]
pub const NEWLINE_OFFSET: usize = 1;
