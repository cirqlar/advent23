pub mod part1;
pub mod part2;

#[cfg(target_os = "windows")]
pub const NEWLINE_OFFSET: usize = 2;
#[cfg(not(target_os = "windows"))]
pub const NEWLINE_OFFSET: usize = 1;

pub fn successors(n: usize, grid_size: usize, input: &[u8]) -> Vec<usize> {
    let actual_grid_size = grid_size + NEWLINE_OFFSET;

    let nx = n % actual_grid_size;
    let ny = n / actual_grid_size;

    let mut nexts = Vec::with_capacity(4);

    if nx > 0 {
        let next = n - 1;
        if input[next] != b'#' {
            nexts.push(next);
        }
    }
    if nx < grid_size - 1 {
        let next = n + 1;
        if input[next] != b'#' {
            nexts.push(next);
        }
    }

    if ny > 0 {
        let next = n - actual_grid_size;
        if input[next] != b'#' {
            nexts.push(next);
        }
    }
    if ny < grid_size - 1 {
        let next = n + actual_grid_size;
        if input[next] != b'#' {
            nexts.push(next);
        }
    }

    nexts
}

pub fn successors_xy(n: &(usize, usize), grid_size: usize, input: &[u8]) -> Vec<(usize, usize)> {
    let actual_grid_size = grid_size + NEWLINE_OFFSET;

    let nx = n.0; // n % actual_grid_size;
    let ny = n.1; // n / actual_grid_size;

    let n = ny * actual_grid_size + nx;

    let mut nexts = Vec::with_capacity(4);

    if nx > 0 {
        let next = n - 1;
        if input[next] != b'#' {
            nexts.push((nx - 1, ny));
        }
    }
    if nx < grid_size - 1 {
        let next = n + 1;
        if input[next] != b'#' {
            nexts.push((nx + 1, ny));
        }
    }

    if ny > 0 {
        let next = n - actual_grid_size;
        if input[next] != b'#' {
            nexts.push((nx, ny - 1));
        }
    }
    if ny < grid_size - 1 {
        let next = n + actual_grid_size;
        if input[next] != b'#' {
            nexts.push((nx, ny + 1));
        }
    }

    nexts
}
