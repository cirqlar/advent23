use itertools::Itertools;
use pathfinding::prelude::bfs;
use rustc_hash::FxHashMap;
use std::str::from_utf8;

pub fn process(input: &[u8], grid_size: usize, count: usize) -> String {
    let mut b_count = 0;
    let bytes_on_the_moon = input
        .split(|ch| ch.is_ascii_whitespace() || ch == &b',')
        .filter(|n| !n.is_empty())
        .tuples()
        .enumerate()
        .map(|(i, (x, y))| {
            let x = from_utf8(x).expect("x").parse::<usize>().expect("x");
            let y = from_utf8(y).expect("y").parse::<usize>().expect("y");

            b_count += 1;

            (y * (grid_size + 1) + x, i)
        })
        .collect::<FxHashMap<_, _>>();

    let mut min = count;
    let mut max = b_count;

    let min = loop {
        let check = min + ((max - min) / 2);

        let res = bfs(
            &0,
            |n| {
                let mut possibles = Vec::with_capacity(4);

                let nx = *n % (grid_size + 1);
                let ny = *n / (grid_size + 1);

                if ny > 0 {
                    let possible = *n - (grid_size + 1);
                    if !bytes_on_the_moon.contains_key(&possible)
                        || bytes_on_the_moon.get(&possible).unwrap() > &check
                    {
                        possibles.push(possible);
                    }
                }
                if ny < grid_size {
                    let possible = *n + (grid_size + 1);
                    if !bytes_on_the_moon.contains_key(&possible)
                        || bytes_on_the_moon.get(&possible).unwrap() > &check
                    {
                        possibles.push(possible);
                    }
                }
                if nx > 0 {
                    let possible = *n - 1;
                    if !bytes_on_the_moon.contains_key(&possible)
                        || bytes_on_the_moon.get(&possible).unwrap() > &check
                    {
                        possibles.push(possible);
                    }
                }
                if nx < grid_size {
                    let possible = *n + 1;
                    if !bytes_on_the_moon.contains_key(&possible)
                        || bytes_on_the_moon.get(&possible).unwrap() > &check
                    {
                        possibles.push(possible);
                    }
                }

                // println!("From {}/{},{} the possibles are {:?}", n, nx, ny, possibles);

                possibles
            },
            |n| *n == (grid_size + 1).pow(2) - 1,
        );

        if res.is_none() {
            max = check;
        } else {
            min = check;
        }

        if max - min <= 1 {
            break max;
        }
    };

    let ans = bytes_on_the_moon
        .into_iter()
        .find_map(|(k, v)| (v == min).then_some(k))
        .unwrap();

    format!("{},{}", ans % (grid_size + 1), ans / (grid_size + 1))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");
        let grid_size = 6;
        let count = 12;

        let answer = super::process(input, grid_size, count);

        assert_eq!(answer, "6,1");
    }
}
