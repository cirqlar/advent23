use std::collections::VecDeque;

use crate::NEWLINE_OFFSET;

pub fn process(input: &[u8], grid_size: usize) -> usize {
    let mut queue = input
        .iter()
        .enumerate()
        .filter_map(|(index, ch)| {
            if ch == &b'0' {
                Some((index, index))
            } else {
                None
            }
        })
        .collect::<VecDeque<_>>();

    let mut counts = vec![0; input.len()];

    while let Some((start, current)) = queue.pop_front() {
        let current_x = current % (grid_size + NEWLINE_OFFSET);
        let current_y = current / (grid_size + NEWLINE_OFFSET);
        let current_ch = input[current];

        // check nexts
        // up
        if current_y > 0 {
            let next_index = current - (grid_size + NEWLINE_OFFSET);
            let next_ch = input[next_index];
            if next_ch - current_ch == 1 {
                if next_ch == b'9' {
                    counts[start] += 1;
                } else {
                    queue.push_back((start, next_index));
                }
            }
        }
        // down
        if current_y < grid_size - 1 {
            let next_index = current + (grid_size + NEWLINE_OFFSET);
            let next_ch = input[next_index];
            if next_ch - current_ch == 1 {
                if next_ch == b'9' {
                    counts[start] += 1;
                } else {
                    queue.push_back((start, next_index));
                }
            }
        }
        // left
        if current_x > 0 {
            let next_index = current - 1;
            let next_ch = input[next_index];
            if next_ch - current_ch == 1 {
                if next_ch == b'9' {
                    counts[start] += 1;
                } else {
                    queue.push_back((start, next_index));
                }
            }
        }
        // right
        if current_x < grid_size - 1 {
            let next_index = current + 1;
            let next_ch = input[next_index];
            if next_ch - current_ch == 1 {
                if next_ch == b'9' {
                    counts[start] += 1;
                } else {
                    queue.push_back((start, next_index));
                }
            }
        }
    }

    counts.into_iter().sum()
}
