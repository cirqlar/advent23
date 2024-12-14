use std::str::from_utf8;

pub fn process(input: &[u8], wide: isize, tall: isize) -> isize {
    // let half_wide = wide / 2;
    // let half_tall = tall / 2;

    let input = input
        .split(|ch| ch == &b'\n' || ch == &b'\r')
        .filter(|line| !line.is_empty())
        .map(|line| {
            // do stuff
            let p_comma = line.iter().position(|ch| ch == &b',').unwrap();
            let px = from_utf8(&line[2..p_comma])
                .expect("px")
                .parse::<isize>()
                .expect("px");
            let f_space = line.iter().position(|ch| ch == &b' ').unwrap();
            let py = from_utf8(&line[(p_comma + 1)..f_space])
                .expect("py")
                .parse::<isize>()
                .expect("py");

            let v_comma = f_space + (line[f_space..]).iter().position(|ch| ch == &b',').unwrap();
            let vx = from_utf8(&line[(f_space + 3)..v_comma])
                .expect("vx")
                .parse::<isize>()
                .expect("vx");
            let vy = from_utf8(&line[(v_comma + 1)..])
                .expect("vy")
                .parse::<isize>()
                .expect("vy");

            ((px, py), (vx, vy))
        })
        .collect::<Vec<_>>();

    let mut finals;

    let mut times = 0;
    'outer: loop {
        times += 1;
        finals = input
            .iter()
            .map(|((px, py), (vx, vy))| {
                let mut nx = (px + (vx * times)) % wide;
                let mut ny = (py + (vy * times)) % tall;

                if nx < 0 {
                    nx += wide
                }
                if ny < 0 {
                    ny += tall
                }

                (nx, ny)
            })
            .collect::<Vec<_>>();

        let mut checker = Vec::with_capacity(finals.len());

        for f in finals.iter() {
            if checker.contains(f) {
                continue 'outer;
            } else {
                checker.push(*f);
            }
        }

        break;
    }

    // Print tree
    for y in 0..tall {
        for x in 0..wide {
            let count = finals.iter().filter(|a| a == &&(x, y)).count();
            if count > 0 {
                print!("{}", count);
            } else {
                print!(".");
            }
        }
        println!();
    }

    times
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn test_1() {
//         let input = include_bytes!("../../input/part1_example.txt");
//         let wide = 11;
//         let tall = 7;

//         let answer = super::process(input, wide, tall);

//         assert_eq!(answer, 12);
//     }
// }
