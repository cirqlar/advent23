use std::{collections::HashMap, str::from_utf8};

#[cfg(target_os = "windows")]
pub const NEWLINE_OFFSET: usize = 2;
#[cfg(not(target_os = "windows"))]
pub const NEWLINE_OFFSET: usize = 1;

pub type RuleMap<T> = HashMap<T, Vec<T>>;
pub const ARR_WIDTH: usize = 25;
pub const ARR_WIDTH_U16: u16 = 25;
const ARR_SIZE: usize = 90 * ARR_WIDTH;
pub type RuleMap2<T> = [Option<T>; ARR_SIZE];

pub fn parse_rules<T: std::str::FromStr + std::cmp::Eq + std::hash::Hash + std::clone::Clone>(
    input: &[u8],
    split_line_number: usize,
) -> RuleMap<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let rules = &input[..(5 + NEWLINE_OFFSET) * (split_line_number - 1)];

    let mut rule_map: RuleMap<T> = HashMap::new();

    rules
        .chunks_exact(5 + NEWLINE_OFFSET)
        .map(|line| {
            (
                from_utf8(&line[..2])
                    .expect("Should be stringable")
                    .parse::<T>()
                    .expect("Should be numberable"),
                from_utf8(&line[3..5])
                    .expect("Should be stringable")
                    .parse::<T>()
                    .expect("Should be numberable"),
            )
        })
        .for_each(|(before, after)| {
            rule_map
                .entry(after.clone())
                .and_modify(|before_list| before_list.push(before.clone()))
                .or_insert(vec![before.clone()]);
        });

    rule_map
}

pub fn parse_rules_2<
    T: std::str::FromStr
        + std::cmp::Eq
        + std::hash::Hash
        + std::clone::Clone
        + std::ops::Sub<u16>
        + std::marker::Copy,
>(
    input: &[u8],
    split_line_number: usize,
) -> RuleMap2<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    <T as std::ops::Sub<u16>>::Output: std::ops::Mul<u16>,
    <<T as std::ops::Sub<u16>>::Output as std::ops::Mul<u16>>::Output: TryInto<usize>,
    <<<T as std::ops::Sub<u16>>::Output as std::ops::Mul<u16>>::Output as std::convert::TryInto<
        usize,
    >>::Error: std::fmt::Debug,
{
    let rules = &input[..(5 + NEWLINE_OFFSET) * (split_line_number - 1)];

    let mut rule_map: RuleMap2<T> = [None; ARR_SIZE];

    rules
        .chunks_exact(5 + NEWLINE_OFFSET)
        .map(|line| {
            (
                from_utf8(&line[..2])
                    .expect("Should be stringable")
                    .parse::<T>()
                    .expect("Should be numberable"),
                from_utf8(&line[3..5])
                    .expect("Should be stringable")
                    .parse::<T>()
                    .expect("Should be numberable"),
            )
        })
        .for_each(|(before, after)| {
            let index: usize = ((after - 10) * ARR_WIDTH_U16).try_into().unwrap();

            let nonedex = rule_map[index..].iter().position(|a| a.is_none()).unwrap();

            rule_map[nonedex] = Some(before);
        });

    rule_map
}
