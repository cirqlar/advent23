use std::io::BufRead;

pub fn process(input: &[u8]) -> i32 {
    let width = input.lines().next().unwrap().unwrap().len();
    let height = input.lines().count();

    let mut count = 0;

    let input = input
        .iter()
        .filter_map(|ch| {
            if ch.is_ascii_whitespace() {
                None
            } else {
                Some(*ch)
            }
        })
        .collect::<Vec<_>>();

    input.iter().enumerate().for_each(|(ind, ch)| {
        if *ch != b'X' {
            return;
        }

        let x_coord = ind % width;
        let y_coord = ind / width;

        // Forward
        if x_coord < width - 3 && input[ind..(ind + 4)] == *b"XMAS" {
            // println!("Found XMAS at {},{}", x_coord, y_coord);
            count += 1;
        }

        // Backward
        if x_coord > 2 && input[(ind - 3)..=ind] == *b"SAMX" {
            // println!("Found SAMX at {},{}", x_coord, y_coord);
            count += 1;
        }

        // Downward
        if y_coord < height - 3
            && input[ind + width] == b'M'
            && input[ind + width * 2] == b'A'
            && input[ind + width * 3] == b'S'
        {
            // println!("Found downward at {},{}", x_coord, y_coord);
            count += 1;
        }

        // Upward
        if y_coord > 2
            && input[ind - width] == b'M'
            && input[ind - width * 2] == b'A'
            && input[ind - width * 3] == b'S'
        {
            // println!("Found upward at {},{}", x_coord, y_coord);
            count += 1;
        }

        // Down Forward
        if x_coord < width - 3
            && y_coord < height - 3
            && input[ind + width + 1] == b'M'
            && input[ind + width * 2 + 2] == b'A'
            && input[ind + width * 3 + 3] == b'S'
        {
            // println!("Found down forward at {},{}", x_coord, y_coord);
            count += 1;
        }

        // Down Back
        if x_coord > 2
            && y_coord < height - 3
            && input[ind + width - 1] == b'M'
            && input[ind + width * 2 - 2] == b'A'
            && input[ind + width * 3 - 3] == b'S'
        {
            // println!("Found down backward at {},{}", x_coord, y_coord);
            count += 1;
        }

        // Up Forward
        if x_coord < width - 3
            && y_coord > 2
            && input[ind - width + 1] == b'M'
            && input[ind - width * 2 + 2] == b'A'
            && input[ind - width * 3 + 3] == b'S'
        {
            // println!("Found up forward at {},{}", x_coord, y_coord);
            count += 1;
        }

        // Up Back
        if x_coord > 2
            && y_coord > 2
            && input[ind - width - 1] == b'M'
            && input[ind - width * 2 - 2] == b'A'
            && input[ind - width * 3 - 3] == b'S'
        {
            // println!("Found up backward at {},{}", x_coord, y_coord);
            count += 1;
        }
    });

    count
}

const NEWLINE_OFFSET: usize = 2;

pub fn process_flatter(input: &[u8]) -> i32 {
    let width = input.lines().next().unwrap().unwrap().len();
    let height = input.lines().count();

    let mut count = 0;

    input
        .iter()
        .enumerate()
        .filter(|(_, ch)| !ch.is_ascii_whitespace())
        .enumerate()
        .for_each(|(ind, (real_ind, ch))| {
            if *ch != b'X' {
                return;
            }

            let x_coord = ind % width;
            let y_coord = ind / width;

            // Forward
            if x_coord < width - 3 && input[real_ind..(real_ind + 4)] == *b"XMAS" {
                // println!("Found XMAS at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Backward
            if x_coord > 2 && input[(real_ind - 3)..=real_ind] == *b"SAMX" {
                // println!("Found SAMX at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Downward
            if y_coord < height - 3
                && input[real_ind + (width + NEWLINE_OFFSET)] == b'M'
                && input[real_ind + (width + NEWLINE_OFFSET) * 2] == b'A'
                && input[real_ind + (width + NEWLINE_OFFSET) * 3] == b'S'
            {
                // println!("Found downward at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Upward
            if y_coord > 2
                && input[real_ind - (width + NEWLINE_OFFSET)] == b'M'
                && input[real_ind - (width + NEWLINE_OFFSET) * 2] == b'A'
                && input[real_ind - (width + NEWLINE_OFFSET) * 3] == b'S'
            {
                // println!("Found upward at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Down Forward
            if x_coord < width - 3
                && y_coord < height - 3
                && input[real_ind + (width + NEWLINE_OFFSET) + 1] == b'M'
                && input[real_ind + (width + NEWLINE_OFFSET) * 2 + 2] == b'A'
                && input[real_ind + (width + NEWLINE_OFFSET) * 3 + 3] == b'S'
            {
                // println!("Found down forward at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Down Back
            if x_coord > 2
                && y_coord < height - 3
                && input[real_ind + (width + NEWLINE_OFFSET) - 1] == b'M'
                && input[real_ind + (width + NEWLINE_OFFSET) * 2 - 2] == b'A'
                && input[real_ind + (width + NEWLINE_OFFSET) * 3 - 3] == b'S'
            {
                // println!("Found down backward at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Up Forward
            if x_coord < width - 3
                && y_coord > 2
                && input[real_ind - (width + NEWLINE_OFFSET) + 1] == b'M'
                && input[real_ind - (width + NEWLINE_OFFSET) * 2 + 2] == b'A'
                && input[real_ind - (width + NEWLINE_OFFSET) * 3 + 3] == b'S'
            {
                // println!("Found up forward at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Up Back
            if x_coord > 2
                && y_coord > 2
                && input[real_ind - (width + NEWLINE_OFFSET) - 1] == b'M'
                && input[real_ind - (width + NEWLINE_OFFSET) * 2 - 2] == b'A'
                && input[real_ind - (width + NEWLINE_OFFSET) * 3 - 3] == b'S'
            {
                // println!("Found up backward at {},{}", x_coord, y_coord);
                count += 1;
            }
        });

    count
}

pub fn process_flatpar(input: &[u8]) -> i32 {
    use rayon::prelude::*;

    let width = input.lines().next().unwrap().unwrap().len();
    let height = input.lines().count();

    input
        .par_iter()
        .enumerate()
        .filter(|(_, ch)| !ch.is_ascii_whitespace())
        .map(|(real_ind, ch)| {
            if *ch != b'X' {
                return 0;
            }

            let x_coord = real_ind % (width + NEWLINE_OFFSET);
            let y_coord = real_ind / (width + NEWLINE_OFFSET);

            let mut count = 0;

            // Forward
            if x_coord < width - 3 && input[real_ind..(real_ind + 4)] == *b"XMAS" {
                // println!("Found XMAS at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Backward
            if x_coord > 2 && input[(real_ind - 3)..=real_ind] == *b"SAMX" {
                // println!("Found SAMX at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Downward
            if y_coord < height - 3
                && input[real_ind + (width + NEWLINE_OFFSET)] == b'M'
                && input[real_ind + (width + NEWLINE_OFFSET) * 2] == b'A'
                && input[real_ind + (width + NEWLINE_OFFSET) * 3] == b'S'
            {
                // println!("Found downward at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Upward
            if y_coord > 2
                && input[real_ind - (width + NEWLINE_OFFSET)] == b'M'
                && input[real_ind - (width + NEWLINE_OFFSET) * 2] == b'A'
                && input[real_ind - (width + NEWLINE_OFFSET) * 3] == b'S'
            {
                // println!("Found upward at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Down Forward
            if x_coord < width - 3
                && y_coord < height - 3
                && input[real_ind + (width + NEWLINE_OFFSET) + 1] == b'M'
                && input[real_ind + (width + NEWLINE_OFFSET) * 2 + 2] == b'A'
                && input[real_ind + (width + NEWLINE_OFFSET) * 3 + 3] == b'S'
            {
                // println!("Found down forward at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Down Back
            if x_coord > 2
                && y_coord < height - 3
                && input[real_ind + (width + NEWLINE_OFFSET) - 1] == b'M'
                && input[real_ind + (width + NEWLINE_OFFSET) * 2 - 2] == b'A'
                && input[real_ind + (width + NEWLINE_OFFSET) * 3 - 3] == b'S'
            {
                // println!("Found down backward at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Up Forward
            if x_coord < width - 3
                && y_coord > 2
                && input[real_ind - (width + NEWLINE_OFFSET) + 1] == b'M'
                && input[real_ind - (width + NEWLINE_OFFSET) * 2 + 2] == b'A'
                && input[real_ind - (width + NEWLINE_OFFSET) * 3 + 3] == b'S'
            {
                // println!("Found up forward at {},{}", x_coord, y_coord);
                count += 1;
            }

            // Up Back
            if x_coord > 2
                && y_coord > 2
                && input[real_ind - (width + NEWLINE_OFFSET) - 1] == b'M'
                && input[real_ind - (width + NEWLINE_OFFSET) * 2 - 2] == b'A'
                && input[real_ind - (width + NEWLINE_OFFSET) * 3 - 3] == b'S'
            {
                // println!("Found up backward at {},{}", x_coord, y_coord);
                count += 1;
            }

            count
        })
        .sum()
}
