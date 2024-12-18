use itertools::Itertools;
use pathfinding::prelude::bfs;
use std::str::from_utf8;

pub fn process(input: &[u8], grid_size: usize, count: usize) -> String {
    use rayon::prelude::*;

    let bytes_on_the_moon = input
        .split(|ch| ch.is_ascii_whitespace() || ch == &b',')
        .filter(|n| !n.is_empty())
        .tuples()
        .map(|(x, y)| {
            let x = from_utf8(x).expect("x").parse::<usize>().expect("x");
            let y = from_utf8(y).expect("y").parse::<usize>().expect("y");

            // let index = y * (grid_size + 1) + x;

            y * (grid_size + 1) + x
        })
        .collect::<Vec<_>>();

    let min = (count..bytes_on_the_moon.len())
        .into_par_iter()
        .filter(|r| {
            let res = bfs(
                &0,
                |n| {
                    let mut possibles = Vec::with_capacity(4);

                    let nx = *n % (grid_size + 1);
                    let ny = *n / (grid_size + 1);

                    if ny > 0 {
                        let possible = *n - (grid_size + 1);
                        if !bytes_on_the_moon[..=*r].contains(&possible) {
                            possibles.push(possible);
                        }
                    }
                    if ny < grid_size {
                        let possible = *n + (grid_size + 1);
                        if !bytes_on_the_moon[..=*r].contains(&possible) {
                            possibles.push(possible);
                        }
                    }
                    if nx > 0 {
                        let possible = *n - 1;
                        if !bytes_on_the_moon[..=*r].contains(&possible) {
                            possibles.push(possible);
                        }
                    }
                    if nx < grid_size {
                        let possible = *n + 1;
                        if !bytes_on_the_moon[..=*r].contains(&possible) {
                            possibles.push(possible);
                        }
                    }

                    // println!("From {}/{},{} the possibles are {:?}", n, nx, ny, possibles);

                    possibles
                },
                |n| *n == (grid_size + 1).pow(2) - 1,
            );

            res.is_none()
        })
        .min()
        .expect("A minimum");

    let ans = bytes_on_the_moon[min];

    format!("{},{}", ans % (grid_size + 1), ans / (grid_size + 1))
}

pub fn process_bfs(input: &[u8], grid_size: usize, count: usize) -> String {
    use rayon::prelude::*;

    let bytes_on_the_moon = input
        .split(|ch| ch.is_ascii_whitespace() || ch == &b',')
        .filter(|n| !n.is_empty())
        .tuples()
        .map(|(x, y)| {
            let x = from_utf8(x).expect("x").parse::<usize>().expect("x");
            let y = from_utf8(y).expect("y").parse::<usize>().expect("y");

            // let index = y * (grid_size + 1) + x;

            y * (grid_size + 1) + x
        })
        .collect::<Vec<_>>();

    let min = (count..bytes_on_the_moon.len())
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|r| {
            let res = bfs(
                &0,
                |n| {
                    let mut possibles = Vec::with_capacity(4);

                    let nx = *n % (grid_size + 1);
                    let ny = *n / (grid_size + 1);

                    if ny > 0 {
                        let possible = *n - (grid_size + 1);
                        if !bytes_on_the_moon[..=*r].contains(&possible) {
                            possibles.push(possible);
                        }
                    }
                    if ny < grid_size {
                        let possible = *n + (grid_size + 1);
                        if !bytes_on_the_moon[..=*r].contains(&possible) {
                            possibles.push(possible);
                        }
                    }
                    if nx > 0 {
                        let possible = *n - 1;
                        if !bytes_on_the_moon[..=*r].contains(&possible) {
                            possibles.push(possible);
                        }
                    }
                    if nx < grid_size {
                        let possible = *n + 1;
                        if !bytes_on_the_moon[..=*r].contains(&possible) {
                            possibles.push(possible);
                        }
                    }

                    // println!("From {}/{},{} the possibles are {:?}", n, nx, ny, possibles);

                    possibles
                },
                |n| *n == (grid_size + 1).pow(2) - 1,
            );

            res.is_none()
        })
        .expect("A minimum");

    let ans = bytes_on_the_moon[min];

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
