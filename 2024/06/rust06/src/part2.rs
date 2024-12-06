use crate::Direction;

pub fn process(input: &[u8], grid_size: usize, possibles: &[usize]) -> i32 {
    use rayon::prelude::*;

    let start_index = input
        .iter()
        .position(|ch| ch == &b'^')
        .expect("There's a guard");

    possibles
        .par_iter()
        .filter(|obs_ind| {
            let mut current_direction = Direction::Up;

            let mut visited_positions = vec![[false, false, false, false]; grid_size * grid_size];
            let up_ind: usize = (&current_direction).into();
            visited_positions[start_index][up_ind] = true;

            let mut x_pos: i32 = (start_index % grid_size).try_into().unwrap();
            let mut y_pos: i32 = (start_index / grid_size).try_into().unwrap();

            let grid_size = grid_size.try_into().unwrap();

            loop {
                let dir_ind: usize = (&current_direction).into();

                let (new_x, new_y) = match current_direction {
                    Direction::Up => (x_pos, y_pos - 1),
                    Direction::Right => (x_pos + 1, y_pos),
                    Direction::Down => (x_pos, y_pos + 1),
                    Direction::Left => (x_pos - 1, y_pos),
                };

                if !(new_x >= 0 && new_y >= 0 && new_x < grid_size && new_y < grid_size) {
                    return false;
                }

                let new_index: usize = (new_y * grid_size + new_x).try_into().unwrap();

                if input[new_index] == b'#' || &&new_index == obs_ind {
                    current_direction = current_direction.next();
                } else {
                    if visited_positions[new_index][dir_ind] {
                        return true;
                    }

                    x_pos = new_x;
                    y_pos = new_y;

                    visited_positions[new_index][dir_ind] = true;
                }
            }
        })
        .count()
        .try_into()
        .unwrap()

    // let mut current_direction = Direction::Up;

    // let mut visited_positions = vec![false; grid_size * grid_size];
    // visited_positions[start_index] = true;

    // let mut x_pos: i32 = (start_index % grid_size).try_into().unwrap();
    // let mut y_pos: i32 = (start_index / grid_size).try_into().unwrap();

    // let grid_size = grid_size.try_into().unwrap();

    // loop {
    //     let (new_x, new_y) = match current_direction {
    //         Direction::Up => (x_pos, y_pos - 1),
    //         Direction::Right => (x_pos + 1, y_pos),
    //         Direction::Down => (x_pos, y_pos + 1),
    //         Direction::Left => (x_pos - 1, y_pos),
    //     };

    //     if !(new_x >= 0 && new_y >= 0 && new_x < grid_size && new_y < grid_size) {
    //         break;
    //     }

    //     let new_index: usize = (new_y * grid_size + new_x).try_into().unwrap();

    //     if input[new_index] == b'#' {
    //         current_direction = current_direction.next();
    //     } else {
    //         x_pos = new_x;
    //         y_pos = new_y;

    //         visited_positions[new_index] = true;
    //     }
    // }

    // visited_positions
    //     .iter()
    //     .filter(|b| **b)
    //     .count()
    //     .try_into()
    //     .unwrap()
}
