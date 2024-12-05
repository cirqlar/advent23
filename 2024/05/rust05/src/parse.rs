use std::{collections::HashMap, str::from_utf8};

#[cfg(target_os = "windows")]
pub const NEWLINE_OFFSET: usize = 2;
#[cfg(not(target_os = "windows"))]
pub const NEWLINE_OFFSET: usize = 1;

pub type RuleMap<T> = HashMap<T, (Vec<T>, Vec<T>)>;

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
                .entry(before.clone())
                .and_modify(|(_, after_list)| after_list.push(after.clone()))
                .or_insert((vec![], vec![after.clone()]));
            rule_map
                .entry(after.clone())
                .and_modify(|(before_list, _)| before_list.push(before.clone()))
                .or_insert((vec![before.clone()], vec![]));
        });

    rule_map
}
