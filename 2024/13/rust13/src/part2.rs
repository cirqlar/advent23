use itertools::Itertools;
use std::str::from_utf8;

pub fn process(input: &[u8]) -> isize {
    input
        .split(|ch| ch == &b'\n' || ch == &b'\r')
        .filter(|line| !line.is_empty())
        .tuples()
        .map(|(a, b, p)| {
            let xa = from_utf8(&a[12..14])
                .expect("Can do")
                .parse::<isize>()
                .expect("Is a number");
            let ya = from_utf8(&a[18..20])
                .expect("Can do")
                .parse::<isize>()
                .expect("Is a number");

            let xb = from_utf8(&b[12..14])
                .expect("Can do")
                .parse::<isize>()
                .expect("Is a number");
            let yb = from_utf8(&b[18..20])
                .expect("Can do")
                .parse::<isize>()
                .expect("Is a number");

            let comma_pos = 9 + p[9..]
                .iter()
                .position(|ch| ch == &b',')
                .expect("There is one");
            let xp = from_utf8(&p[9..comma_pos])
                .expect("Can do")
                .parse::<isize>()
                .expect("Is a number")
                + 10000000000000;
            let yp = from_utf8(&p[(comma_pos + 4)..])
                .expect("Can do")
                .parse::<isize>()
                .expect("Is a number")
                + 10000000000000;

            let d = (xa * yb) - (ya * xb);
            let da = (xp * yb) - (yp * xb);
            let a = da / d;
            let a_rem = da % d;
            let db = (xa * yp) - (ya * xp);
            let b = db / d;
            let b_rem = db % d;

            if b >= 0 && b_rem == 0 && a >= 0 && a_rem == 0 {
                // println!("We got a {} and b {}", a, b);
                a * 3 + b
            } else {
                // println!("No solution with a {} and b {}", a, b);
                0
            }
        })
        .sum()
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn test_1() {
//         let input = b"Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

// Button A: X+26, Y+66
// Button B: X+67, Y+21
// Prize: X=12748, Y=12176

// Button A: X+17, Y+86
// Button B: X+84, Y+37
// Prize: X=7870, Y=6450

// Button A: X+69, Y+23
// Button B: X+27, Y+71
// Prize: X=18641, Y=10279";

//         let answer = super::process(input);

//         assert_eq!(answer, 480);
//     }
// }
