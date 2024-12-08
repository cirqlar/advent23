use crate::parse::ANTENNA_STRIDE;
use itertools::Itertools;

pub fn process(antenna_keys: &[u8], antenna_vals: &[Option<usize>], grid_size: usize) -> usize {
    antenna_keys
        .iter()
        .enumerate()
        .flat_map(|(key_index, _key)| {
            let start = key_index * ANTENNA_STRIDE;
            let end = start + ANTENNA_STRIDE;

            antenna_vals[start..end]
                .iter()
                .flatten()
                .tuple_combinations()
                .flat_map(|(a, b)| {
                    let ax_pos = a % grid_size;
                    let ay_pos = a / grid_size;

                    let bx_pos = b % grid_size;
                    let by_pos = b / grid_size;

                    let x_dist = ax_pos.abs_diff(bx_pos);
                    let y_dist = ay_pos.abs_diff(by_pos);

                    let mut valid_indices = vec![*a, *b];

                    // a side
                    let mut times = 1;
                    loop {
                        let a_side_x = if ax_pos < bx_pos {
                            if ax_pos >= (x_dist * times) {
                                Some(ax_pos - (x_dist * times))
                            } else {
                                None
                            }
                        } else {
                            let npos = ax_pos + (x_dist * times);
                            if npos < grid_size {
                                Some(npos)
                            } else {
                                None
                            }
                        };
                        let a_side_y = if ay_pos < by_pos {
                            if ay_pos >= (y_dist * times) {
                                Some(ay_pos - (y_dist * times))
                            } else {
                                None
                            }
                        } else {
                            let npos = ay_pos + (y_dist * times);
                            if npos < grid_size {
                                Some(npos)
                            } else {
                                None
                            }
                        };

                        if let (Some(x), Some(y)) = (a_side_x, a_side_y) {
                            valid_indices.push(y * grid_size + x);
                            times += 1;
                        } else {
                            break;
                        }
                    }

                    // b side
                    let mut times = 1;
                    loop {
                        let b_side_x = if bx_pos < ax_pos {
                            if bx_pos >= (x_dist * times) {
                                Some(bx_pos - (x_dist * times))
                            } else {
                                None
                            }
                        } else {
                            let npos = bx_pos + (x_dist * times);
                            if npos < grid_size {
                                Some(npos)
                            } else {
                                None
                            }
                        };
                        let b_side_y = if by_pos < ay_pos {
                            if by_pos >= (y_dist * times) {
                                Some(by_pos - (y_dist * times))
                            } else {
                                None
                            }
                        } else {
                            let npos = by_pos + (y_dist * times);
                            if npos < grid_size {
                                Some(npos)
                            } else {
                                None
                            }
                        };

                        if let (Some(x), Some(y)) = (b_side_x, b_side_y) {
                            valid_indices.push(y * grid_size + x);
                            times += 1;
                        } else {
                            break;
                        }
                    }

                    valid_indices
                })
        })
        .unique()
        .count()
}
