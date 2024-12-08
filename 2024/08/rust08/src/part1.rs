use crate::{check_and_count, parse::ANTENNA_STRIDE};
use itertools::Itertools;

pub fn process(
    antenna_keys: &[u8],
    antenna_vals: &[Option<usize>],
    grid_size: usize,
    input_len: usize,
) -> usize {
    let mut check = vec![false; input_len];
    let mut count = 0;

    antenna_keys
        .iter()
        .enumerate()
        .flat_map(|(key_index, _key)| {
            let start = key_index * ANTENNA_STRIDE;
            let end = start + ANTENNA_STRIDE;

            antenna_vals[start..end]
                .iter()
                .flatten()
                .tuple_combinations::<(_, _)>()
        })
        .for_each(|(a, b)| {
            let ax_pos = a % grid_size;
            let ay_pos = a / grid_size;

            let bx_pos = b % grid_size;
            let by_pos = b / grid_size;

            let x_dist = ax_pos.abs_diff(bx_pos);
            let y_dist = ay_pos.abs_diff(by_pos);

            // a side
            let a_side_x = if ax_pos < bx_pos {
                if ax_pos >= x_dist {
                    Some(ax_pos - x_dist)
                } else {
                    None
                }
            } else {
                let npos = ax_pos + x_dist;
                if npos < grid_size {
                    Some(npos)
                } else {
                    None
                }
            };
            let a_side_y = if ay_pos < by_pos {
                if ay_pos >= y_dist {
                    Some(ay_pos - y_dist)
                } else {
                    None
                }
            } else {
                let npos = ay_pos + y_dist;
                if npos < grid_size {
                    Some(npos)
                } else {
                    None
                }
            };

            if let (Some(x), Some(y)) = (a_side_x, a_side_y) {
                check_and_count(y * grid_size + x, &mut check, &mut count);
            }

            // b side
            let b_side_x = if bx_pos < ax_pos {
                if bx_pos >= x_dist {
                    Some(bx_pos - x_dist)
                } else {
                    None
                }
            } else {
                let npos = bx_pos + x_dist;
                if npos < grid_size {
                    Some(npos)
                } else {
                    None
                }
            };
            let b_side_y = if by_pos < ay_pos {
                if by_pos >= y_dist {
                    Some(by_pos - y_dist)
                } else {
                    None
                }
            } else {
                let npos = by_pos + y_dist;
                if npos < grid_size {
                    Some(npos)
                } else {
                    None
                }
            };

            if let (Some(x), Some(y)) = (b_side_x, b_side_y) {
                check_and_count(y * grid_size + x, &mut check, &mut count);
            }
        });

    count
}
