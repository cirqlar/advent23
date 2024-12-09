pub fn process(input: &[u8]) -> usize {
    let mut map = input
        .iter()
        .enumerate()
        .map(|(index, ch)| {
            let ch = ch - b'0';
            let mut num: isize = -1;

            if index % 2 == 0 {
                num = (index / 2).try_into().unwrap();
            }

            (num, ch)
        })
        .collect::<Vec<_>>();

    let mut back_index = map.len() - 1;

    'outer: loop {
        let (back_val, back_size) = map[back_index];
        if back_val != -1 {
            for front_index in 1..back_index {
                let (front_val, front_size) = map[front_index];

                if front_val == -1 && front_size == back_size {
                    map[front_index] = (back_val, back_size);
                    map[back_index] = (-1, back_size);

                    break; // falls through to back index set at bottom
                } else if front_val == -1 && front_size > back_size {
                    let size_diff = front_size - back_size;

                    map[front_index] = (back_val, back_size);
                    map[back_index] = (-1, back_size);
                    map.insert(front_index + 1, (-1, size_diff));

                    continue 'outer; // skips back index set because increasing the size array means we need to check the same position again.
                }
            }
        }

        if back_index == 0 {
            break;
        } else {
            back_index -= 1;
        }
    }

    let mut sum = 0;
    let mut index: isize = 0;

    for (val, size) in map {
        if val != -1 {
            let nsize: isize = size.into();
            for i in index..(index + nsize) {
                let n: usize = (val * i).try_into().unwrap();
                sum += n;
            }
        }
        let n: isize = size.into();
        index += n;
    }

    sum
}

// pub fn process_2(input: &[u8]) -> usize {
//     let mut map: Vec<(isize, u8)> = Vec::with_capacity(20264);
//     let mut moved = vec![false; input.len()];

//     for (index, ch) in input.iter().enumerate() {
//         if index % 2 == 0 {
//             map.push((
//                 if moved[index] {
//                     -1
//                 } else {
//                     (index / 2).try_into().unwrap()
//                 },
//                 ch - b'0',
//             ));
//         } else {
//             let mut size = ch - b'0';
//             let mut start = input.len();
//             if start % 2 == 0 {
//                 start -= 1;
//             }

//             'outer: while size > 0 {
//                 for i in ((index + 1)..start).rev().step_by(2) {
//                     if moved[i] {
//                         continue;
//                     }

//                     let moving_size = input[i] - b'0';
//                     if moving_size <= size {
//                         moved[i] = true;
//                         map.push(((i / 2).try_into().unwrap(), moving_size));
//                         size -= moving_size;
//                         continue 'outer;
//                     }
//                 }

//                 map.push((-1, size));
//                 break;
//             }
//         }
//     }

//     let mut sum = 0;
//     let mut index: isize = 0;

//     for (val, size) in map {
//         if val != -1 {
//             let nsize: isize = size.into();
//             for i in index..(index + nsize) {
//                 let n: usize = (val * i).try_into().unwrap();
//                 sum += n;
//             }
//         }
//         let n: isize = size.into();
//         index += n;
//     }

//     sum
// }

// pub fn process_2(input: &[u8]) -> usize {
//     let mut map = Vec::with_capacity(94595); // I checked how many
//                                              // let mut map: Vec<(isize, u8)> = Vec::with_capacity(20264);
//     let mut moved = vec![false; input.len()];

//     for (index, ch) in input.iter().enumerate() {
//         if index % 2 == 0 {
//             let num: isize = if moved[index] {
//                 -1
//             } else {
//                 (index / 2).try_into().unwrap()
//             };
//             (0..(ch - b'0')).for_each(|_| {
//                 map.push(num);
//             });
//         } else {
//             let mut size = ch - b'0';
//             let mut start = input.len();
//             if start % 2 == 0 {
//                 start -= 1;
//             }

//             'outer: while size > 0 {
//                 for i in ((index + 1)..start).rev().step_by(2) {
//                     if moved[i] {
//                         continue;
//                     }

//                     let moving_size = input[i] - b'0';
//                     if moving_size <= size {
//                         moved[i] = true;
//                         let num: isize = (i / 2).try_into().unwrap();
//                         (0..moving_size).for_each(|_| {
//                             map.push(num);
//                         });
//                         size -= moving_size;
//                         continue 'outer;
//                     }
//                 }

//                 (0..size).for_each(|_| {
//                     map.push(-1);
//                 });
//                 break;
//             }
//         }
//     }

//     map.into_iter()
//         .enumerate()
//         .filter_map(|(ind, n)| n.try_into().ok().map(|o| (ind, o)))
//         .map(|(index, n): (_, usize)| index * n)
//         .sum()
// }

pub fn do_not_space(index: usize, ch: u8, map: &mut Vec<isize>, moved: &mut [bool]) {
    let num: isize = if moved[index] {
        -1
    } else {
        (index / 2).try_into().unwrap()
    };
    (0..(ch - b'0')).for_each(|_| {
        map.push(num);
    });
}

pub fn do_space(
    index: usize,
    ch: u8,
    map: &mut Vec<isize>,
    moved: &mut [bool],
    most_moved: &mut usize,
    input: &[u8],
) {
    let mut size = ch - b'0';
    let start = *most_moved;

    'outer: while size > 0 {
        for i in ((index + 1)..start).rev().step_by(2) {
            if moved[i] {
                continue;
            }

            let moving_size = input[i] - b'0';
            if moving_size <= size {
                let num: isize = (i / 2).try_into().unwrap();
                (0..moving_size).for_each(|_| {
                    map.push(num);
                });

                // Advance
                moved[i] = true;
                size -= moving_size;
                if *most_moved - i < 2 {
                    *most_moved = i - 1;
                }

                continue 'outer;
            }
        }

        (0..size).for_each(|_| {
            map.push(-1);
        });
        break;
    }
}

pub fn process_2(input: &[u8]) -> usize {
    let mut map = Vec::with_capacity(94595); // I checked how many
                                             // let mut map: Vec<(isize, u8)> = Vec::with_capacity(20264);
    let mut moved = vec![false; input.len()];

    let mut most_moved = input.len();

    for (index, ch) in input.iter().enumerate() {
        if index % 2 == 0 {
            do_not_space(index, *ch, &mut map, &mut moved);
        } else {
            do_space(index, *ch, &mut map, &mut moved, &mut most_moved, input);
        }
    }

    map.into_iter()
        .enumerate()
        .filter_map(|(ind, n)| n.try_into().ok().map(|o| (ind, o)))
        .map(|(index, n): (_, usize)| index * n)
        .sum()
}
