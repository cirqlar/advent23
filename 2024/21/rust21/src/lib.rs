pub mod fns;
pub mod part1;
pub mod part2;
mod reda;

pub use reda::*;

#[cfg(target_os = "windows")]
pub const NEWLINE_OFFSET: usize = 2;
#[cfg(not(target_os = "windows"))]
pub const NEWLINE_OFFSET: usize = 1;
