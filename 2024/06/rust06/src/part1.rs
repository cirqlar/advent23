use crate::Direction;

pub fn process(input: &[u8], grid_size: usize) -> i32 {
    let start_index = input
        .iter()
        .position(|ch| ch == &b'^')
        .expect("There's a guard");
    let mut current_direction = Direction::Up;

    let mut visited_positions = vec![false; grid_size * grid_size];
    visited_positions[start_index] = true;

    let mut x_pos: i32 = (start_index % grid_size).try_into().unwrap();
    let mut y_pos: i32 = (start_index / grid_size).try_into().unwrap();

    let grid_size = grid_size.try_into().unwrap();

    loop {
        let (new_x, new_y) = match current_direction {
            Direction::Up => (x_pos, y_pos - 1),
            Direction::Right => (x_pos + 1, y_pos),
            Direction::Down => (x_pos, y_pos + 1),
            Direction::Left => (x_pos - 1, y_pos),
        };

        if !(new_x >= 0 && new_y >= 0 && new_x < grid_size && new_y < grid_size) {
            break;
        }

        let new_index: usize = (new_y * grid_size + new_x).try_into().unwrap();

        if input[new_index] == b'#' {
            current_direction = current_direction.next();
        } else {
            x_pos = new_x;
            y_pos = new_y;

            visited_positions[new_index] = true;
        }
    }

    visited_positions
        .iter()
        .filter(|b| **b)
        .count()
        .try_into()
        .unwrap()
}

pub fn process_return(input: &[u8], grid_size: usize) -> (i32, Vec<usize>) {
    let start_index = input
        .iter()
        .position(|ch| ch == &b'^')
        .expect("There's a guard");
    let mut current_direction = Direction::Up;

    let mut visited_positions = vec![false; grid_size * grid_size];
    visited_positions[start_index] = true;

    let mut x_pos: i32 = (start_index % grid_size).try_into().unwrap();
    let mut y_pos: i32 = (start_index / grid_size).try_into().unwrap();

    let grid_size = grid_size.try_into().unwrap();

    loop {
        let (new_x, new_y) = match current_direction {
            Direction::Up => (x_pos, y_pos - 1),
            Direction::Right => (x_pos + 1, y_pos),
            Direction::Down => (x_pos, y_pos + 1),
            Direction::Left => (x_pos - 1, y_pos),
        };

        if !(new_x >= 0 && new_y >= 0 && new_x < grid_size && new_y < grid_size) {
            break;
        }

        let new_index: usize = (new_y * grid_size + new_x).try_into().unwrap();

        if input[new_index] == b'#' {
            current_direction = current_direction.next();
        } else {
            x_pos = new_x;
            y_pos = new_y;

            visited_positions[new_index] = true;
        }
    }

    visited_positions
        .iter()
        .enumerate()
        .filter(|(ind, b)| **b && ind != &start_index)
        .fold((1, vec![]), |mut acc, (ind, _)| {
            acc.0 += 1;
            acc.1.push(ind);
            acc
        })
}
