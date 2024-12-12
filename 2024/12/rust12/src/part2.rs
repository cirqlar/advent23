use std::collections::VecDeque;

use crate::NEWLINE_OFFSET;

pub fn process(input: &[u8], grid_size: usize) -> usize {
    let actual_grid_size = grid_size + NEWLINE_OFFSET;

    let mut visited = vec![false; input.len()];

    let mut totals = vec![];

    for (index, ch) in input.iter().enumerate() {
        if ch.is_ascii_whitespace() || visited[index] {
            continue;
        }

        // Store Plot type
        let plot_type = *ch;

        // Make queue to check and mark us as visited
        let mut plot_queue = VecDeque::from([index]);
        visited[index] = true;

        // Make a new total for us and store a mut ref
        totals.push((1, vec![]));
        let last_index = totals.len() - 1;
        let our_total = totals.get_mut(last_index).expect("We just made it");

        // Check neighbours and add to the area/primeter when found/not found
        while let Some(plot) = plot_queue.pop_front() {
            let plot_x = plot % actual_grid_size;
            let plot_y = plot / actual_grid_size;

            let mut our_perimeters = [false; 4];

            // check up
            if plot_y > 0 {
                let next_plot = plot - actual_grid_size;
                if input[next_plot] == plot_type {
                    if !visited[next_plot] {
                        plot_queue.push_back(next_plot);
                        visited[next_plot] = true;
                        our_total.0 += 1;
                    }
                } else {
                    our_perimeters[0] = true;
                }
            } else {
                our_perimeters[0] = true;
            }

            // check down
            if plot_y < grid_size - 1 {
                let next_plot = plot + actual_grid_size;
                if input[next_plot] == plot_type {
                    if !visited[next_plot] {
                        plot_queue.push_back(next_plot);
                        visited[next_plot] = true;
                        our_total.0 += 1;
                    }
                } else {
                    our_perimeters[2] = true;
                }
            } else {
                our_perimeters[2] = true;
            }

            // check left
            if plot_x > 0 {
                let next_plot = plot - 1;
                if input[next_plot] == plot_type {
                    if !visited[next_plot] {
                        plot_queue.push_back(next_plot);
                        visited[next_plot] = true;
                        our_total.0 += 1;
                    }
                } else {
                    our_perimeters[3] = true;
                }
            } else {
                our_perimeters[3] = true;
            }

            // check right
            if plot_x < grid_size - 1 {
                let next_plot = plot + 1;
                if input[next_plot] == plot_type {
                    if !visited[next_plot] {
                        plot_queue.push_back(next_plot);
                        visited[next_plot] = true;
                        our_total.0 += 1;
                    }
                } else {
                    our_perimeters[1] = true;
                }
            } else {
                our_perimeters[1] = true;
            }

            if our_perimeters[0] {
                if our_perimeters[1] {
                    our_total.1.push((plot_x + 1, plot_y));
                } else if plot_y > 0 && plot_x < grid_size - 1 {
                    let up_right_index = plot - (actual_grid_size - 1);
                    if input[up_right_index] == plot_type {
                        let corner = (plot_x + 1, plot_y);
                        if !our_total.1.contains(&corner) {
                            our_total.1.push(corner);
                        }
                    }
                }
                if our_perimeters[3] {
                    our_total.1.push((plot_x, plot_y));
                } else if plot_y > 0 && plot_x > 0 {
                    let up_left_index = plot - (actual_grid_size + 1);
                    if input[up_left_index] == plot_type {
                        let corner = (plot_x, plot_y);
                        if !our_total.1.contains(&corner) {
                            our_total.1.push(corner);
                        }
                    }
                }
            }
            if our_perimeters[1] {
                if !our_perimeters[0] && plot_y > 0 && plot_x < grid_size - 1 {
                    let up_right_index = plot - (actual_grid_size - 1);
                    if input[up_right_index] == plot_type {
                        let corner = (plot_x + 1, plot_y);
                        if !our_total.1.contains(&corner) {
                            our_total.1.push(corner);
                        }
                    }
                }
                if !our_perimeters[2] && plot_y < grid_size - 1 && plot_x < grid_size - 1 {
                    let down_right_index = plot + (actual_grid_size + 1);
                    if input[down_right_index] == plot_type {
                        let corner = (plot_x + 1, plot_y + 1);
                        if !our_total.1.contains(&corner) {
                            our_total.1.push(corner);
                        }
                    }
                }
            }
            if our_perimeters[2] {
                if our_perimeters[1] {
                    our_total.1.push((plot_x + 1, plot_y + 1));
                } else if plot_y < grid_size - 1 && plot_x < grid_size - 1 {
                    let down_right_index = plot + (actual_grid_size + 1);
                    if input[down_right_index] == plot_type {
                        let corner = (plot_x + 1, plot_y + 1);
                        if !our_total.1.contains(&corner) {
                            our_total.1.push(corner);
                        }
                    }
                }
                if our_perimeters[3] {
                    our_total.1.push((plot_x, plot_y + 1));
                } else if plot_y > grid_size - 1 && plot_x > 0 {
                    let down_left_index = plot + (actual_grid_size - 1);
                    if input[down_left_index] == plot_type {
                        let corner = (plot_x, plot_y + 1);
                        if !our_total.1.contains(&corner) {
                            our_total.1.push(corner);
                        }
                    }
                }
            }
            if our_perimeters[3] {
                if !our_perimeters[0] && plot_y > 0 && plot_x > 0 {
                    let up_left_index = plot - (actual_grid_size + 1);
                    if input[up_left_index] == plot_type {
                        let corner = (plot_x, plot_y);
                        if !our_total.1.contains(&corner) {
                            our_total.1.push(corner);
                        }
                    }
                }
                if !our_perimeters[2] && plot_y > grid_size - 1 && plot_x > 0 {
                    let down_left_index = plot + (actual_grid_size - 1);
                    if input[down_left_index] == plot_type {
                        let corner = (plot_x, plot_y + 1);
                        if !our_total.1.contains(&corner) {
                            our_total.1.push(corner);
                        }
                    }
                }
            }
        }
    }

    // println!("{:?}", totals);

    totals
        .into_iter()
        .map(|(area, perimeter)| area * perimeter.len())
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = b"AAAA
BBCD
BBCC
EEEC";
        let grid_size = 4;

        let answer = super::process(input, grid_size);

        assert_eq!(answer, 80);
    }

    #[test]
    fn test_2() {
        let input = b"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let grid_size = 5;

        let answer = super::process(input, grid_size);

        assert_eq!(answer, 236);
    }

    #[test]
    fn test_3() {
        let input = b"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let grid_size = 10;

        let answer = super::process(input, grid_size);

        assert_eq!(answer, 1206);
    }
}
