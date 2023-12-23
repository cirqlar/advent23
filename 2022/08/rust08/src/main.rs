fn main() {
    let input = include_str!("../../input.txt");
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut visible = (height * 2) + (width * 2) - 4;

    let input = input
        .chars()
        .filter_map(|c| c.to_string().parse::<u8>().ok())
        .collect::<Vec<_>>();


    println!("Visible {visible}");

    for index in 0..(input.len()-visible) {
        let row = (index / (width -2)) + 1;
        let row_index = index % (width-2);
        let actual_index = (row * width) + 1 + row_index;

        // Check up
        let is_not_visible = (0..row).any(|i| {
            input[actual_index] <= input[actual_index - (width * (row - i))]
        });
        if !is_not_visible {
            visible += 1;
            continue;
        }
        // Check down
        let is_not_visible = ((row+1)..height).any(|i| {
            input[actual_index] <= input[actual_index + (width * (i - row))]
        });
        if !is_not_visible {
            visible += 1;
            continue;
        }
        // Check left
        let is_not_visible = ((row*width)..actual_index).any(|i| {
            input[actual_index] <= input[i]
        });
        if !is_not_visible {
            visible += 1;
            continue;
        }
        // Check right
        let is_not_visible = ((actual_index+1)..((row+1)*width)).any(|i| {
            input[actual_index] <= input[i]
        });
        if !is_not_visible {
            visible += 1;
            continue;
        }
    }

    println!("Visible after checks {}", visible);
    
    let highest = (0..(input.len()-visible)).fold(
        0,
        |mut acc, index| {
            let row = (index / (width -2)) + 1;
            let row_index = index % (width-2);
            let actual_index = (row * width) + 1 + row_index;

            // Check up
            let mut views;
            let mut total = 0;
            for i in (0..row).rev() {
                total += 1;
                if input[actual_index] <= input[actual_index - (width * (row - i))] {
                    break;
                }
            };
            views = total;
            // Check down
            let mut total = 0;
            for i in (row+1)..height {
                total += 1;
                if input[actual_index] <= input[actual_index + (width * (i - row))] {
                    break;
                }
            };
            views *= total;
            // Check left
            let mut total = 0;
            for i in ((row*width)..actual_index).rev() {
                total += 1;
                if input[actual_index] <= input[i] {
                    break;
                }
            };
            views *= total;
            // Check right
            let mut total = 0;
            for i in (actual_index+1)..((row+1)*width) {
                total += 1;
                if input[actual_index] <= input[i] {
                    break;
                }
            };
            views *= total;

            if acc < views {
                acc = views;
            }

            acc
        }
    );

    println!("The highest {highest}");
}
