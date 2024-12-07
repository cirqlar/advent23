pub mod part1;
pub mod part2;

pub fn num_digits(mut num: u64) -> u32 {
    let mut n = 1;

    // if num >= 100000000 {
    //     n += 8;
    //     num /= 100000000;
    // }
    if num >= 10000 {
        n += 4;
        num /= 10000;
    }
    if num >= 100 {
        n += 2;
        num /= 100;
    }
    if num >= 10 {
        n += 1;
    }

    n
}
