use std::str::from_utf8;

pub fn process(input: &[u8], wide: isize, tall: isize) -> isize {
    let times = 100;
    let half_wide = wide / 2;
    let half_tall = tall / 2;

    let finals = input
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

            let mut nx = (px + (vx * times)) % wide;
            let mut ny = (py + (vy * times)) % tall;

            if nx < 0 {
                nx += wide
            }
            if ny < 0 {
                ny += tall
            }

            match (nx, ny) {
                (x, y) if x < half_wide && y < half_tall => 1,
                (x, y) if x < half_wide && y > half_tall => 2,
                (x, y) if x > half_wide && y < half_tall => 3,
                (x, y) if x > half_wide && y > half_tall => 4,
                _ => 0,
            }
        })
        .fold([0, 0, 0, 0, 0], |mut acc, item| {
            acc[item] += 1;
            acc
        });

    finals.into_iter().skip(1).product()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let input = include_bytes!("../../input/part1_example.txt");
        let wide = 11;
        let tall = 7;

        let answer = super::process(input, wide, tall);

        assert_eq!(answer, 12);
    }
}
