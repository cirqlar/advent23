use std::{collections::HashMap, iter};

fn main() {
    let input = include_str!("../../../input/part1.txt");
    let answer = process(input);
    println!("Answer {answer}");
}

fn transform_north(vecs: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut iters = vecs
        .into_iter()
        .map(|row: Vec<char>| row.into_iter())
        .collect::<Vec<_>>();

    iter::from_fn(|| {
        let mut items = Vec::new();

        for iter in &mut iters {
            match iter.next() {
                Some(item) => items.push(item),
                None => return None,
            }
        }

        Some(items)
    })
    .collect::<Vec<_>>()
}

fn transform_west(vecs: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut iters = vecs
        .into_iter()
        .map(|col| col.into_iter())
        .collect::<Vec<_>>();

    iter::from_fn(|| {
        let mut items = Vec::new();

        for iter in &mut iters {
            match iter.next() {
                Some(item) => items.push(item),
                None => return None,
            }
        }

        Some(items)
    })
    .collect::<Vec<_>>()
}

type Cache = HashMap<Vec<char>, Vec<char>>;

fn shift_north(vecs: &mut [Vec<char>], cache: &mut Cache) {
    for v in vecs {
        if cache.contains_key(v) {
            *v = cache.get(v).expect("has").clone();
            continue;
        }
        let mut first_availabe = 0;
        let key = v.clone();

        for index in 0..v.len() {
            if v[index] == '.' {
                continue;
            }
            if v[index] == 'O' {
                if first_availabe != index {
                    v[first_availabe] = 'O';
                    v[index] = '.';
                }
                first_availabe += 1;
            }
            if v[index] == '#' {
                first_availabe = index + 1;
            }
        }

        cache.insert(key, v.clone());
    }
}

fn shift_south(vecs: &mut [Vec<char>], cache: &mut Cache) {
    for v in vecs {
        let mut key = v.clone();
        key.reverse();
        if cache.contains_key(&key) {
            *v = cache.get(&key).expect("has").clone();
            v.reverse();
            continue;
        }
        let mut first_availabe = v.len() - 1;

        for index in (0..v.len()).rev() {
            if v[index] == '.' {
                continue;
            }
            if v[index] == 'O' {
                if first_availabe != index {
                    v[first_availabe] = 'O';
                    v[index] = '.';
                }
                if index != 0 {
                    first_availabe -= 1;
                }
            }
            if v[index] == '#' && index != 0 {
                first_availabe = index - 1;
            }
        }

        cache.insert(key, v.clone().into_iter().rev().collect());
    }
}

fn shift_west(vecs: Vec<Vec<char>>, cache: &mut Cache) -> Vec<Vec<char>> {
    let mut vecs = transform_west(vecs);

    for v in vecs.iter_mut() {
        if cache.contains_key(v) {
            *v = cache.get(v).expect("has").clone();
            continue;
        }
        let mut first_availabe = 0;
        let key = v.clone();

        for index in 0..v.len() {
            if v[index] == '.' {
                continue;
            }
            if v[index] == 'O' {
                if first_availabe != index {
                    v[first_availabe] = 'O';
                    v[index] = '.';
                }
                first_availabe += 1;
            }
            if v[index] == '#' {
                first_availabe = index + 1;
            }
        }

        cache.insert(key, v.clone());
    }

    transform_north(vecs)
}

fn shift_east(vecs: Vec<Vec<char>>, cache: &mut Cache) -> Vec<Vec<char>> {
    let mut vecs = transform_west(vecs);

    for v in vecs.iter_mut() {
        let mut key = v.clone();
        key.reverse();
        if cache.contains_key(&key) {
            *v = cache.get(&key).expect("has").clone();
            v.reverse();
            continue;
        }
        let mut first_availabe = v.len() - 1;

        for index in (0..v.len()).rev() {
            if v[index] == '.' {
                continue;
            }
            if v[index] == 'O' {
                if first_availabe != index {
                    v[first_availabe] = 'O';
                    v[index] = '.';
                }
                if index != 0 {
                    first_availabe -= 1;
                }
            }
            if v[index] == '#' && index != 0 {
                first_availabe = index - 1;
            }
        }

        cache.insert(key, v.clone().into_iter().rev().collect());
    }

    transform_north(vecs)
}

fn process(input: &str) -> usize {
    let mut iters = input.lines().map(|line| line.chars()).collect::<Vec<_>>();

    let mut columns = iter::from_fn(|| {
        let mut items = Vec::new();

        for iter in &mut iters {
            match iter.next() {
                Some(item) => items.push(item),
                None => return None,
            }
        }

        Some(items)
    })
    .collect::<Vec<_>>();

    let mut cache: Cache = HashMap::new();
    let mut prevs = Vec::new();

    for i in 0..1000000000 {
        shift_north(&mut columns, &mut cache);
        columns = shift_west(columns, &mut cache);
        shift_south(&mut columns, &mut cache);
        columns = shift_east(columns, &mut cache);

        if prevs.contains(&columns) {
            let ind = prevs
                .iter()
                .position(|a| a == &columns)
                .expect("can't get here otherwise");
            let repeat = i - ind;
            let left = 1000000000 - (i + 1);
            let rem = left % repeat;

            columns = prevs[rem + ind].clone();
            break;
        }

        prevs.push(columns.clone());
    }

    columns
        .iter()
        .flat_map(|v| {
            let length = v.len();
            v.iter().enumerate().filter_map(move |(index, ch)| {
                if *ch != 'O' {
                    None
                } else {
                    Some(length - index)
                }
            })
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d14_part_2_is_correct() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(process(input), 64);
    }
}
