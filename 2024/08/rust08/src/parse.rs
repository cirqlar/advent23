use std::collections::HashMap;

pub fn process(input: &[u8]) -> (Vec<u8>, HashMap<u8, Vec<usize>>) {
    let mut antennas = HashMap::new();

    let input = input
        .iter()
        .filter(|ch| !ch.is_ascii_whitespace())
        .enumerate()
        .map(|(index, ch)| {
            if ch != &b'.' {
                antennas
                    .entry(*ch)
                    .and_modify(|indices: &mut Vec<usize>| indices.push(index))
                    .or_insert(vec![index]);
            }

            *ch
        })
        .collect::<Vec<_>>();

    (input, antennas)
}

pub const ANTENNA_STRIDE: usize = 4;

pub fn process_2(input: &[u8]) -> (Vec<u8>, Vec<Option<usize>>) {
    let mut antenna_keys = vec![];
    let mut antenna_vals = vec![];

    input
        .iter()
        .filter(|ch| !ch.is_ascii_whitespace())
        .enumerate()
        .for_each(|(index, ch)| {
            if ch != &b'.' {
                if !antenna_keys.contains(ch) {
                    antenna_keys.push(*ch);
                    antenna_vals.append(&mut vec![None; ANTENNA_STRIDE]);
                }
                let ind = antenna_keys.iter().position(|a| a == ch).unwrap();
                let start = ind * ANTENNA_STRIDE;

                let set_ind = start
                    + antenna_vals[start..(start + ANTENNA_STRIDE)]
                        .iter()
                        .position(|el| el.is_none())
                        .unwrap();

                antenna_vals[set_ind] = Some(index);
            }
        });

    (antenna_keys, antenna_vals)
}
