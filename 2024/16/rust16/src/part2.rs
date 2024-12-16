use crate::{
    direction::{AddDirection, Direction},
    NEWLINE_OFFSET,
};
use pathfinding::prelude::astar_bag_collect;

pub fn process(input: &[u8], grid_size: usize) -> usize {
    let actual_grid_size = grid_size + NEWLINE_OFFSET;
    let start_index = ((grid_size - 2) * actual_grid_size) + 1; //bottom left corner
    let start_direction = Direction::Right; // east

    let end_index = actual_grid_size + (grid_size - 2); // top right corner
    let end_x = grid_size - 2;
    let end_y = 1;

    let start_pos = (start_index, start_direction);
    let res = astar_bag_collect(
        &start_pos,
        |pos| {
            let mut nexts = vec![];

            // Forward
            let forward_index = pos.0.add_direction(&pos.1, actual_grid_size);
            if input[forward_index] != b'#' {
                nexts.push(((forward_index, pos.1), 1));
            }

            // counter_clockwise turn
            let new_dir = pos.1.counter_clockwise();
            nexts.push(((pos.0, new_dir), 1000));

            // clockwise turn
            let new_dir = pos.1.clockwise();
            nexts.push(((pos.0, new_dir), 1000));

            nexts
        },
        |pos| {
            let pos_x = pos.0 % actual_grid_size;
            let pos_y = pos.0 / actual_grid_size;

            (pos_x.abs_diff(end_x) + pos_y.abs_diff(end_y)) / 3
        },
        |pos| pos.0 == end_index,
    )
    .expect("Should find a path");

    // for y in 0..grid_size {
    //     for x in 0..grid_size {
    //         let index = y * actual_grid_size + x;

    //         if res.0.iter().any(|(i, _d)| i == &index) {
    //             print!("A");
    //         } else {
    //             print!("{}", input[index] as char);
    //         }
    //     }
    //     println!();
    // }

    let mut all_possibles = vec![];

    res.0.into_iter().flatten().for_each(|(p, _)| {
        if !all_possibles.contains(&p) {
            all_possibles.push(p);
        }
    });

    all_possibles.len()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");
        let grid_size = 15;

        let answer = super::process(input, grid_size);

        assert_eq!(answer, 45);
    }

    #[test]
    fn test_2() {
        let input = include_bytes!("../../input/part2_example.txt");
        let grid_size = 17;

        let answer = super::process(input, grid_size);

        assert_eq!(answer, 64);
    }
}
