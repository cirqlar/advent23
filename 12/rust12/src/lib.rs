use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub type Hash = HashMap<(Vec<char>, Vec<usize>), usize>;
pub type Cache = Arc<RwLock<Hash>>;

pub fn check_cached(input: &[char], counts: &[usize], cache: &Cache) -> usize {
    let key = &(input.to_vec(), counts.to_vec());
    if let Some(x) = cache.read().unwrap().get(key) {
        return *x;
    }

    let starting_positions = input
        .windows(counts[0])
        .enumerate()
        .filter_map(|(index, v)| {
            if v.iter().all(|ch| *ch != '.') {
                Some(index)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if starting_positions.is_empty() {
        return 0;
    }
    let mut count = 0;

    for pos in starting_positions {
        if !input[0..pos].contains(&'#')
            && (pos + counts[0] >= input.len() || input[pos + counts[0]] != '#')
        {
            if counts.len() == 1 {
                if !input[(pos + counts[0])..input.len()]
                    .iter()
                    .any(|c| *c == '#')
                {
                    count += 1;
                }

                if !input[pos..(pos + counts[0])].contains(&'?')
                    && (pos + counts[0] >= input.len() || input[pos + counts[0]] == '.')
                {
                    break;
                } else {
                    continue;
                }
            }

            if pos + counts[0] + 1 >= input.len() {
                continue;
            }

            let value = check_cached(&input[(pos + counts[0] + 1)..], &counts[1..], cache);
            let key = (
                input[(pos + counts[0] + 1)..].to_vec(),
                counts[1..].to_vec(),
            );
            count += value;
            if !cache.read().unwrap().contains_key(&key) {
                cache.write().unwrap().insert(key, value);
            }
            if !input[pos..(pos + counts[0])].contains(&'?')
                && (pos + counts[0] >= input.len() || input[pos + counts[0]] == '.')
            {
                break;
            }
        }
    }

    count
}
