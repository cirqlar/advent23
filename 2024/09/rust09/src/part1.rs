pub fn process(input: &[u8]) -> usize {
    let mut free_list = [0_usize; 44516]; // I checked how many
    let mut free_list_counter: usize = 0; // counts current pos in free list
    let mut index_counter: usize = 0; // counts current pos in main list

    let mut map = vec![-1_isize; 94595]; // I checked how many

    input.iter().enumerate().for_each(|(index, ch)| {
        if !ch.is_ascii_whitespace() {
            let ch = ch - b'0';

            let n: usize = ch.into();
            if index % 2 == 0 {
                let num = (index / 2).try_into().unwrap();
                for el in map.iter_mut().skip(index_counter).take(n) {
                    *el = num;
                }

                index_counter += n;
            } else {
                for i in index_counter..(index_counter + n) {
                    free_list[free_list_counter] = i;
                    free_list_counter += 1;
                }
                index_counter += n;
            };
        }
    });

    let mut front_index;
    let mut back_index = map.len() - 1;

    let mut free_list_index = 0;
    'outer: loop {
        front_index = free_list[free_list_index];
        // PreAdvance
        free_list_index += 1;

        for i in (0..=back_index).rev() {
            if i < front_index {
                break 'outer;
            }

            if map[i] != -1 {
                back_index = i;
                break;
            }
        }

        map[front_index] = map[back_index];
        map[back_index] = -1;
    }

    map.into_iter()
        .filter_map(|n| n.try_into().ok())
        .enumerate()
        .map(|(index, n): (_, usize)| index * n)
        .sum()
}
