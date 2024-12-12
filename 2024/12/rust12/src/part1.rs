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
        totals.push((1, 0));
        let last_index = totals.len() - 1;
        let our_total = totals.get_mut(last_index).expect("We just made it");

        // Check neighbours and add to the area/primeter when found/not found
        while let Some(plot) = plot_queue.pop_front() {
            let plot_x = plot % actual_grid_size;
            let plot_y = plot / actual_grid_size;

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
                    our_total.1 += 1;
                }
            } else {
                our_total.1 += 1;
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
                    our_total.1 += 1;
                }
            } else {
                our_total.1 += 1;
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
                    our_total.1 += 1;
                }
            } else {
                our_total.1 += 1;
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
                    our_total.1 += 1;
                }
            } else {
                our_total.1 += 1;
            }
        }
    }

    totals
        .into_iter()
        .map(|(area, perimeter)| area * perimeter)
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

        assert_eq!(answer, 140);
    }

    #[test]
    fn test_2() {
        let input = b"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let grid_size = 5;

        let answer = super::process(input, grid_size);

        assert_eq!(answer, 772);
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

        assert_eq!(answer, 1930);
    }
}
